import { Transformation } from "axon-client-common/lib/modules/models/transformation/interfaces/transformation";

import { CellOutputType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_output_type";
import { CellInputType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_input_type";
import { CellDepType } from "axon-client-common/lib/modules/models/cells/interfaces/cell_dep_type";
import { WitnessInputType } from "axon-client-common/lib/modules/models/witnesses/interfaces/witness_input_type";
import { GenericTransformation } from "axon-client-common/lib/modules/models/transformation/generic_transformation";

/*
this service compose tx for rpc
 */
export default interface TransactionService {
  composeTransaction(xfer: Transformation): Promise<void>;

  composeTransactionFromGeneric<
    DT extends Array<CellDepType>,
    IT extends Array<CellInputType>,
    OT extends Array<CellOutputType>,
    WT extends WitnessInputType | undefined,
  >(
    xfer: GenericTransformation<DT, IT, OT, WT>,
  ): Promise<void>;
}
