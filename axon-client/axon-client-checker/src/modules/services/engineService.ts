import { CheckerSubmitTaskTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_task";
import { CheckSubmitChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_submit_challenge";
import { CheckPublishChallengeTransformation } from "axon-client-common/src/modules/models/transformation/checker_publish_challenge";

export default interface EngineService {
  checkerSubmitTask(xfer: CheckerSubmitTaskTransformation): Promise<void>;

  checkerSubmitChallenge(xfer: CheckSubmitChallengeTransformation): Promise<void>;

  checkerPublishChallenge(xfer: CheckPublishChallengeTransformation): Promise<void>;
}
