import { Transformation } from "axon-client-common/lib/modules/models/transformation/interfaces/transformation";

/*
this service compose tx for rpc
 */
export default interface TransactionService {
  composeTransaction(xfer: Transformation): void;
}
