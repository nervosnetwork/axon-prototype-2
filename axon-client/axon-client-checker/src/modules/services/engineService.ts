import { CheckerSubmitTaskTransformation } from "axon-client-common/lib/modules/models/transformation/checker_submit_task";
import { CheckerSubmitChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/checker_submit_challenge";
import { CheckerPublishChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/checker_publish_challenge";
import { DeployCodeTransformation } from "axon-client-common/lib/modules/models/transformation/deploy_code_transformation";

export default interface EngineService {
  checkerSubmitTask(xfer: CheckerSubmitTaskTransformation): Promise<void>;

  checkerSubmitChallenge(xfer: CheckerSubmitChallengeTransformation): Promise<void>;

  checkerPublishChallenge(xfer: CheckerPublishChallengeTransformation): Promise<void>;

  checkerDeployCodeCell(transformation: DeployCodeTransformation): Promise<void>;
}
