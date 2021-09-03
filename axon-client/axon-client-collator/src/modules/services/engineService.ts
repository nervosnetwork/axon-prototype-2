import { CollatorPublishTaskTransformation } from "axon-client-common/lib/modules/models/transformation/collator_publish_task";
import { CollatorSubmitTasksTransformation } from "axon-client-common/lib/modules/models/transformation/collator_submit_tasks";
import { CollatorSubmitChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/collator_submit_challenge";
import { CollatorRefreshTaskTransformation } from "axon-client-common/lib/modules/models/transformation/collator_refresh_task";

export default interface EngineService {
  collatorPublishTask(xfer: CollatorPublishTaskTransformation): Promise<void>;

  collatorSubmitTask(xfer: CollatorSubmitTasksTransformation): Promise<void>;

  collatorSubmitChallenge(xfer: CollatorSubmitChallengeTransformation): Promise<void>;

  refreshTask(xfer: CollatorRefreshTaskTransformation): Promise<void>;
}
