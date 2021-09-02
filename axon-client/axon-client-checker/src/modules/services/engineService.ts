import { CheckerVoteTransformation } from "axon-client-common/lib/modules/models/transformation/checker_vote";
import { CheckerPublishChallengeTransformation } from "axon-client-common/lib/modules/models/transformation/checker_publish_challenge";
import { DeployCodeTransformation } from "axon-client-common/lib/modules/models/transformation/deploy_code_transformation";

export default interface EngineService {
  checkerVote(xfer: CheckerVoteTransformation): Promise<void>;

  checkerPublishChallenge(xfer: CheckerPublishChallengeTransformation): Promise<void>;

  checkerDeployCodeCell(transformation: DeployCodeTransformation): Promise<void>;
}
