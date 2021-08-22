# Axon侧链方案四



## Axon 设计稿

### 设计概要,改良重点

1. 加入以commit-reveal为基础的prng
2. 支持并行发布,打包确认,减少确认延迟
3. 采用molecule来序列化data,以便支持多个动态数组
4. 存放近2048个block header hash的数据
5. 添加punishment, 频繁refresh的checker被关进jail,无法赎回bond

### 设计思路

1. 将ckb chain认为是数据库,type认为是数据库的schema,cell data认为是record. code cell作为"trigger".
2. 继续采用链上cell触发的设计思路,放弃链下交互,聚合信息,最后链上提交的伪"rollup"方案.
3. 所有的cell data,都采用molecule序列化,因为要处理多个可变数组的情况下,手动计算偏移量没有必要.

molecule无法*非常直观*的计算出序列化后的大小,所以在对capacity的问题上,采用"最低满足"的策略.


1. checker设定*无*上限,不再采用bitmap.不采用bitmap会在tx额外有所开销,但是微乎其微.
2. chain的设定*无*上限.
3. 支持并行发布(publish),打包提交(submit)~~,跳跃确认(confirm)~~.
4. 在主要核心逻辑task中,内置commit-reveal随机数方案.
5. 在task中,添加超时时间戳,满足refresh操作
6. 针对侧链的一段高度,可以不发commit_threshold个task,这些同一个高度的task,叫做job

##cell 设计

terminology

type Id script : 确保唯一的script code
type Id args : type Id script 的 arg, 此 arg用来确保script group是唯一的
type Id : 整个script,也可以指代type Id script被hash之后的typehash,
address: blake160 of public key


###Global config cell

用以保存所有script的typescript hash,确保正版

#### Data

admin_lock_arg 20 bytes, 方便读取
*code_cell_typescript_hash* 20 bytes
*sidechain_config_cell_typescript_hash* 20 bytes
*sidechain_state_cell_typescript_hash* 20 bytes
*checker_info_cell_typescript_hash* 20 bytes
*checker_bond_cell_typescript_hash* 20 bytes
*task_cell_typescript_hash* 20 bytes
*sidechain_fee_cell_typescript_hash* 20 bytes
*sidechain_bond_cell_typescript_hash* 20 bytes

#### Type
codehash: type Id script code hash

hashtype: type

args: type Id args

#### Lock

code hash: secp256k1

hashtype: type

args: admin_address







###Checker bond cell

checker一次质押,就可以参与所有的侧链
只有当所有的侧链都退出,才可以赎回

#### Data
SUDT data

#### Type
Muse token

#### Lock
script: 确保gcc和code cell是认证的代码

args:


    * checker_address
    * participated_chain_id [u32], sidechain config 的 chain Id的数组

###Sidechain bond cell

collator用于抵押资产到特定侧链的cell

#### Data
SUDT data

#### Type
sidechain custom token

#### Lock
script: 确保gcc和code cell是认证的代码,同时dynamic load secp256k1

args:


    * participated_chain, sidechain config 的 id
    * collator_address, bytes20, 用以解锁secp256k1
    * unlock_sidechain_height, 只有当侧链特定高度被submit后, 才可以赎回

###Sidechain registry cell

用以记录有哪些chain_id被申请了,同时也会记录哪些chain_id被正确/challenge关闭了
只能部署一次,但是代码上没有限制,因为此操作是admin执行的
#### Data
chain_ids : [u32],chainid是一个u32范围内的随机数,且不能和记录中已有的重复

#### Type
script: 确保gcc和code cell是认证的代码

#### Lock
script: always success







### Sidechain config cell

记录特定被开启的侧链,保存配置信息
*update_interval*被暂时取消,因为没有办法方便地获取headerDep

#### Data
sidechain_status: u8, 当前链处于Relaying,Halting,还是Shutdown
checker_total_count : u32, 快速记录当前总共有多少checker被记录
checker_normal_count: u32, 快速记录档当前总共有多少Normal状态的checker
checker_threshold : u32, 记录update侧链状态需要最少多少checker
~~update_interval_timestamp_ms~~: u64, 当前版本不使用,保留,用于记录scc被修改的间隔,单位毫秒
minimal_bond: u128, checker加入该侧链最少需要抵押多少muse token

parallel_job_upper_bond: u8, 最多可以"并行"发布多少任务.
parallel_job_maximal_height_range: u128, 每个任务最多可以跨越多少侧链高度

check_data_size_limit: u128,
check_fee_rate: u32

**shutdown_timeout: u64, 任何人都可以关闭侧链(collator超时)所需的时间**

**refresh_interval: u64, refresh所需的时间**
refresh_punish_point: u32, 每次有被refresh时, 对应的checker被记录的惩罚值,
refresh_punish_threshold: u32, 当一个checker的punish_point达到该值后,会被会被没收cic
refresh_punish_release_point: u32, 当一个checker成功submit task,并且collator也正常submit后,可以减少多少punish_point

commit_threshold: u32, 一次发布需要多少个task
challenge_threshold: u32, 一次task转成challenge后,需要多少个challenge

admin_lock_address: bytes20
collator_lock_address: bytes20
bond_sudt_type_hash: bytes32, sidechain bond 的sudt type hash

activated_checkers: [checker_address: [u8; 20]], 记录了正常的checker,数组上限为232次方, checker_address为checker的secp256k1地址,
jailed_checkers:[checker_address: [u8; 20]], 记录被锁死的checker
#### Type
script: 确保gcc和code cell是认证的代码
args:

    * chain_id, u32, sidechain config 的 id

#### Lock
script: always success







###Sidechain state cell

记录特定被开启的侧链,保存侧链的数据信息
因为可以被并发publish发布, 打包submit提交

E.g.

| task 高度区间 | task 高度区间 |task 高度区间 |task 高度区间 |
| ---- | ---- | ---- |---- |
|  100~109| 110~119 | 120~129|130~139|
| submitted| waiting| waiting| waiting|

需要submit 120~129的job,需要先submit 110~119的job

#### Data
version: u8

submitted_sidechain_block_height: u128, 在上面的例子中,是109
waiting_jobs: [(u128,u128)], 在上面的例子中是[(110,119)]
~~confirmed_jobs~~

prng_seed: bytes20, 最后一次更新的伪随机数种子
prng_offset: u8,最后一次使用上述种子的次数

prng_commit: [(bytes20, bytes32)], 记录上一次所有的commit, 以供下一次reveal的时候校验.在打包submit的时候,需要处理中间的commit-reveal.


punish_checkers: [(bytes20,u32)], 记录了被punish的名单的数量,

recent_block_headers: [bytes20], 最近的block headers, 设想是记录2048个,随后开始循环队列+archive成MPT
ancient_block_heard_merkle_root: bytes20

checker_last_task_sidechain_height: [(bytes20,u128)],记录了某个checker最后一次接受任务的最后侧链高度,如果一个cic想要退出侧链,那么他的这个最后工作高度一定要小于等于submitted_sidechain_block_height
#### Type
script: 确保gcc和code cell是认证的代码

args:


    * chain_id, u32, sidechain config 的 id

#### Lock
script: always success







###Checker info cell

记录了checker加入侧链的信息

#### Data
unpaid_fee: u128, 未提取的muse token, 每次check submit task的时候,都需要连同更新此值

status: u8, Relaying,Quit.

rpc_url: [u8], 动态长度

#### Type
script: 确保gcc和code cell是认证的代码

args:


    * chain_id, u32, sidechain 的 id
    * checker_address, bytes20, 用以解锁secp256k1

#### Lock
script: always success







###Task cell

记录某条侧链某个job的任务信息,同时也承载了prng的作用

#### Data
version: u8
sidechain_block_height_from: u128
sidechain_block_height_to: u128

**refresh_timestamp**: u64, 需要在创建/刷新task的时候,设定何时可以刷新

check_data_size: u128

mode: Task或者是Challenge
status: 是Idle,TaskPass,ChallengePass还是ChallengeRejected

reveal: bytes32, pre-image
commit: bytes20, 下一次reveal的image

sidechain_block_headers: [bytes32], 动态数组,差值为to-from
#### Type
script: 确保gcc和code cell是认证的代码

args:


    * chain_id, u32, sidechain config 的 id
    * checker_address, bytes20, 用以指明是哪个checker的活

#### Lock
script: always success








###Sidechain fee cell

每次collator发布job的时候,需要注入must token,checker可以提取.
提取时,需要校验state内submitted height高于sidechain unlock_height.如果checker一直在处理task,那么他可以同时构造2个tx,先withdraw然后处理task

#### Data

#### Type
script: 确保gcc和其他cell是认证的代码,接受witeness并开始校验逻辑

#### Lock
script: require code cell
args:
chain_id




###Code cell

Schema, 由admin来掌控

#### Data

#### Type
script: 确保gcc和其他cell是认证的代码,接受witeness并开始校验逻辑

#### Lock
script: secp256k1 + admin_address








pattern,即所有tx,包括部署

1 预部署 Global config cell

在3网准备好admin私钥,并已这个私钥的地址为lock args部署一个Global config cell,并将typeId hardcoe写入代码.
所有非code script寻找到Global config cell,判断code cell是否为正版.
code script寻找到Global config cell,判断其他cell的script是否为正版.

考虑到主网和测试网一次部署,终生可用. dev.toml将GCC在genesis部署,应该也是可行.所以只要在编译时,设定GCC的typeId就可以了.







2 pattern: 部署 Code cell

没有什么好说的,任何admin,collator,checker都得部署code cell,多少无所谓.
为了确保code被创建的同时,没有其他的cell蒙混过关

ckb cell                ->      ckb cell
->      code cell







3 pattern: admin deploy sidechain registry cell

注意,只能由admin触发,且只能部署一次,现在registry的记录是空的.这里相信admin不会多次部署

global config cell

code cell               ->      code cell
ckb cell                ->      sidechain registry cell
->      ckb cell

开启一个新的空的registry







4 pattern: admin create sidechain

global config cell

code cell               ->      code cell
sidechain registry cell ->      sidechain registry cell
ckb cell                ->      sidechain config cell
->      sidechain state cell
->      ckb cell
将新的chain_id填入Sidechain registry cell的chain_statuses

初始化state config cell,设定其中的所有参数, checkers数组为空

设定为 state config cell type script args的chain_id为一个随机值,且不能重复

初始化sidechain state cell


    * submitted_block_height为0,或者指定高度
    * waiting_jobs为空
    * ~~confirmed_jobs为空~~
    * prng_seed初始设定为blake160(bytes32(0x00))
    * prng_offset设定为0
    * prng_commit设定为空
    * punish_checkers设定为空
    * recent_block_headers将sidechain genesis block header hash写入,或者写入指定高度
    * ancient_block_heard_merkle_root设定为blake160(bytes32(0x00)),或者将之前所有的block header的merkle root写入

5 pattern: collator publish job

global config cell
sidechain config cell

code cell               ->      code cell
sidechain state cell    ->      sidechain state cell
sidechain bond cell/sudt->      sidechain bond cell
sidechain fee cell/muse ->      sidechain fee cell
ckb cell                ->      [task cell]
->      ckb cell

sidechain bond cell的unlock_sidechain_height需要提高到发布的job的最后一个高度

sidechain fee cell 需要支付已challenge为前提的费用,这些费用可以在submit/settle的时候扣回来

sidechain state cell的需要在把这一次job的高度范围写入到waiting_jobs.

task cell 选取的task,是根据prng_seed和prng_offset做计算,然后在sidechain config cell的checkers中选定的.checker必须是Normal的.
prng_offset在每一次使用后+1.







6 pattern: collator submit tasks

global config cell
[sidechain config cell]

code cell               ->      code cell
[sidechain config cell]->      [sidechain config cell],reveal的值不对
sidechain state cell    ->      sidechain state cell
sidechain fee cell ->      sidechain fee cell
->    muse token
[task cell]            ->      ckb cell
ckb cell

collator可以submit多个job,但是任意一个job被submit之前,他之前的job必须被submit.~~注意处理confirm的情形.也可以一并被submit~~.

sidechain fee cell需要扣除正确的muse token.

sidechain state cell,需要根据task,把waiting_jobs~~和confirmed_jobs~~正确处理到submitted_block_height.
prng_seed需要从所有的task的reveal中重新计算得到.prng_offset清零.如果有checker被前后多次选中task.也要校验commit-reveal.
如果是第一次使用,还没有commit,那么不需要reveal,reveal设定一个默认值
将最新的commit记录到prng_commit.

punish_checkers可以根据减少值.
recent_block_headers追加记录.最新的高度永远放在0的位置.

checker_last_task_sidechain_height记录最新的最后工作高度






**被废弃!!!!**
7 pattern: collator confirm job

global config cell
sidechain config cell

code cell               ->      code cell
sidechain state cell    ->      sidechain state cell
[task cell]            ->      ckb cell
ckb cell

collator可以confirm多个job

sidechain state cell,需要根据task,正确处理waiting_jobs到confirmed_jobs







8 pattern: collator settle challenge

global config cell

code cell               ->      code cell
sidechain config cell   ->      sidechain config cell
sidechain state cell    ->      sidechain state cell
sidechain fee cell/muse ->      sidechain fee cell
[task/challenge cell]  ->      ckb cell
ckb cell

collator一次只能处理1个challenge,减少复杂度

sidechain fee cell需要扣除正确的muse token.即便是challenge的task,也需要支付fee

sidechain state cell,需要根据task,把waiting_jobs~~和confirmed_jobs~~正确处理到submitted_block_height.
prng_seed需要从所有的*有效*task的reveal中重新计算得到.prng_offset清零.
将最新的commit记录到prng_commit.
punish_checkers可以根据减少值.
recent_block_headers追加记录.最新的高度永远放在0的位置.

sidechain config cell, checkers中将发出错误challenge的checker的状态设定为jailed.


9 pattern: collator shutdown sidechain

global config cell
sidechain state cel

sidechain config cell   ->      sidechain config cell

正常关闭侧链,必须当waiting_jobs~~和confirmed_jobs~~都为空的时候

config cell的状态设定为shutdown







10 pattern: collator unlock bond

global config cell
sidechain config cell
sidechain state cell

sidechain bond cell     ->      sudt cell

config cell的状态必须为shutdown
state cell的submit height的高度为bond cell的unlock height







11 pattern: anyone refresh task

[header dep]

global config cell


code cell               ->     code cell     
sidechain config cell   ->      sidechain config cell, 可能被jaid
sidechain state cell    ->      sidechain state cell
task cell               ->      task cell

task利用headerDep的时间戳,感知超时,利用state cell里面的prng,生成新的task cell
state cell追加punish,如果有必要,添加到config的jail.

如果有太多次超时,那么僵尸checker会最终被jail,使得僵尸checker减少,checker命中率大大提升
如果最终checker太少,无法进行下去.有新的checker进来后,继续淘汰僵尸checker,直到满足情形.







12 pattern: anyone shutdown sidechain

[header dep]
global config cell

code cell               ->      code cell
sidechain config cell   ->      sidechain config cell
sidechain state cell    ->      sidechain state cell
[task/challenge cell]  ->      ckb cell
ckb cell

collator一次只能处理1个challenge,减少复杂度

sidechain state cell,将submitted_block_height停留在之前的高度保持不变

sidechain config cell, 将status设定为shutdown,任何cic都可以从shutdown的状态下退出,只要不是jailed

headdep 证明至少经过了config cell的 shutdown_timeout时间





X pattern: checker stake bond

无脑不检查



13 pattern: checker join sidechain

global config cell

code cell               ->      code cell
sidechain config cell   ->      sidechain config cell
checker bond cell       ->      checker bond cell
ckb cell                ->      checker info cell
->      ckb cell






sidechain config cell, 根据config的要求, bond必须满足, config中将checker添加到checkers
checker bond cell, 将config的chainId追加到participated数组
checker info cell, 状态处于Relaying







14 pattern: checker submit task

global config cell
sidechain config cell

code cell               ->      code cell
checker info cell       ->      checker info cell
task cell               ->      task cell
ckb cell                ->      ckb cell

checker info cell的状态必须是Relaying

将info cell正确计费. 因为并行发布,可能在前一个job中,该checker已经被jail了,但是已经选出了task仍然有效,checker可以选择继续工作换取报酬,也可以不工作.
checker bond肯定是无法取出.如果checker选择不工作,那么refresh的时候,有了新的config,重新选择checker的时候就不会再次选中这个人.







15 pattern: checker publish challenge

global config cell
sidechain config cell

code cell               ->      code cell
checker info cell       ->      checker info cell
task cell               ->      [task cell]
ckb cell                ->      ckb cell

checker info cell的状态必须是Relaying

自身的task设定为ChallengeReject,发布其他的Challenge.







16 pattern: checker submit challenge

global config cell
sidechain config cell

code cell               ->      code cell
checker info cell       ->      checker info cell
task cell               ->      task cell
ckb cell                ->      ckb cell

checker info cell的状态必须是Relaying

和 checker submit task一样







17 pattern: checker quit sidechain

global config cell
sidechain state cell

code cell               ->      code cell
sidechain config cell   ->      sidechain config cell
checker bond cell       ->      checker bond cell
checker info cell       ->      checker info cell

checker info cell的状态必须是Relaying

sidechain config cell里面 checker不能被jail,以及如下条件.

如果sidechain config cell处于关闭状态,那么任何checker都可以退出

如果sidechain config cell处于正常运行状态:
sidechain state cell里面 checker不能处于punish列表中,防止恶意攻击.
checker_last_task_sidechain_height中checker_last_height必须小于等于submit_height,
没有被分配新的任务,以免cic被回收后,无法处理新的task,导致refresh.

sidechain config cell里面剔除checker
checker bond cell里面participated_chain_id剔除此链
checker info cell的状态设定为Quit







18 pattern: checker withdraw bond

global config cell

code cell               ->      code cell
checker bond cell       ->      muse cell


checker bond cell没有参与任何侧链







19 pattern: checker take beneficiary

global config cell

code cell               ->      code cell
sidechain fee cell      ->      sidechain fee cell
muse cell               ->      muse cell
checker info cell       ->      [checker info cell]
ckb cell                ->      ckb cell

根据checker info cell中记录的unpaid,从fee cell中划转到muse cell.

如果checker info cell的状态是Quit,则强制回收check info cell.



F.A.Q.

1. Checker 在退出侧链的时候,需要引用sidechain state,并且需要等待他最后的task被submit,是否会增加hot-cell的问题.

会,在task被submit后,可能collator就立马开启新的job.考虑到可以提高cycle fee,被频繁选中的概率不高,以及退出侧链不是一个频繁的操作,所以可以忍受


2. Checker info cell记录了unpaid checker,但是没有任何检查checker是否没有正常工作,也没有确保sidechain fee里面的muse token所覆盖的height一定被submit了

是的,但是这个影响非常的小


1. 首先,在并发发布的情况下,如果collator在submit的时候,才打入对应的fee,那么意味着collator必须走完这个流程,collator有作恶的可能
2. 其次,在不考虑最后一次checker是否作恶的情况下,提前打入和最后结算的效果是一样的.但是如果是最后结算,就需要记录哪些checker可能作恶,就需要在

某些地方记录这种信息,不可能是checker info cell, 因为cic的设计就是为了checker可以并发确认,collator是不操作的,否侧hot-cell太严重.
那只可能是sidecahin state cell或者sidechain config cell. 无论是哪个,都需要处理最后一个正常工作的高度,,然后take beneficiary的时候,
引用sidechain state cell. 这种情况很少发生,虽然会造成hot-cell.但是可以忍受.可是为了明确checker正常工作到哪个高度,需要sidechain state
cell依次记录每一个checker的每一次task的费用,或者当take beneficiary的时候,把部分记录清楚.无论怎么处理,都需要记录至少parallel个记录,以及
海量的未提取的记录.....工作量太大.



1. 然后,checker如果有提交了靠后的waiting的task,那么unpaid fee也会增加,这时checker可能是认真工作了,也可能作恶.但是如果之前的任务导致链
被关停,这部分收益就没了.本着checker只能多拿收益绝不能少拿的原则,collator在publish job的时候,得提前支付fee.无论checker最后的结果如何,fee
都是可以被提取的.


3. Checker在并发任务中,参与了多个任务,且前者的任务因为各种原因被jail了,那么后面的任务如何处理?

这种情况只可能发生在checker在被jailed之前,sidechain config还没有被更新的情况下,使用了sidechain state的prng被选中了.
无论如何,这个checker肯定是被jailed了,都无法退出这个侧链了,checker bond注定被锁死.


1. 如果后者任务在checker被jailed之前就完成了,那么照常,beneficiary也支付也没有问题.剩下的该怎么处理就怎么处理.

   如果再次被jail,那也就只jail一次了.


1. 如果后者任务还没有完成,那么这个checker可以继续完成,结果和上面的一样.
2. 如果后者任务还没有完成,但是这个checker不工作了,触发了refresh.根据新的sidechain config,这个checker已经被jail了,不会再被选中了.

4. checker提交的task/challenge的结果是ok的,但是reveal的pre-image是错的,怎么办?

在这次处理按照正常的节奏处理,对于这个checker的reveal,如果是错误的,那么他可能后续的commit-reveal也被忽略(打包提交).在选取prng的时候,将此pre-image剔除.将checker jail.

为什么是直接jail而不是punish. 这个情况非常严重,因为而已checker可以观测其他的checker reveal的值,如果可以导向到更多的checker.那么他就可以放弃reveal.
每一次这样的攻击,都是一次加剧而已checker聚拢选中的可能性.如果成功,这个概率其实不低,而且如果在成功的基础上,下一次用此概率,又选到了更多的而已checker.那么攻击会马上生效.
未来避免这种势头,直接jail处理,否则这种攻击的成功的平均期望,虽然不高,但是随着时间的累计,最终一定会发生.


~~5. 为什么要设计confirm~~

~~这样是可以看见sidechain的进展,为refresh提供一个"时间"概念. 被confirm的task, 至少task的结论是一致的,reveal的结论尚不可知.task的结论是一致的表示sidechain有了进展,有了"时间"的概念.~~


6. 如何防止collator作恶后无法提取sidechain bond

任何人都可以提交challenge,如果challenge成功,submitted_sidechain_block_height是不会更新的,并且sidechain_status设定为shutdown.
因为collator在publish job的时候,将sidechain bond的unlock_sidechain_height设定在更高的高度,所以永远不可能解锁.


7. 如何使得checker在collator作恶后可以取回checker bond

在sidechain_status为shutdown的情况下,不用去校验checker_last_task_sidechain_height和submitted_sidechain_block_height的关系.确切的说,
checker_last_task_sidechain_height大于submitted_sidechain_block_height也没有关系.同时不用去校验punish的问题,只需要判断是否为jailed就可以了.

如果条件满足,那么在checker bond里面可以剔除这个侧链.


8. 如何确保checker bond在checker作恶后不可能提取出来

被jailed了,怎么都不可能取出来,而且jailed的记录一直在,不会允许你再次加入.如果你换了私钥,那么地址变了,你需要重新质押bond,和之前被锁住的无关了.


9. 我不想重新relay一条运行了许久的侧链,想从中途开始怎么办

submitted_sidechain_block_height, recent_block_headers,ancient_block_heard_merkle_root填入基于高度信息即可.

10. ckb在这些流程中要如何计算

说实话,这个得到写代码的是偶再做计算了,因为确实单独放在设计里,有点不讨好,ckb从总体思路上前后基本保持一致,对于checker而言,是不会有什么额外的ckb收货的.
