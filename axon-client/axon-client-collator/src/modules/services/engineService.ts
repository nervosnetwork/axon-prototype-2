import { CollatorPublishTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_publish_task";
import { CollatorSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_submit_task";
import { CollatorSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/collator_submit_challenge";
import { CollatorRefreshTaskTransformation } from "axon-client-common/src/modules/models/transformation/collator_refresh_task";

export default interface EngineService {
  collatorPublishTask(xfer: CollatorPublishTaskTransformation): Promise<void>;

  collatorSubmitTask(xfer: CollatorSubmitTaskTransformation): Promise<void>;

  collatorSubmitChallenge(xfer: CollatorSubmitChallengeTransformation): Promise<void>;

  refreshTask(xfer: CollatorRefreshTaskTransformation): Promise<void>;
}
