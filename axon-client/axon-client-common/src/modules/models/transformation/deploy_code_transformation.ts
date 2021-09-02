import { GenericTransformation } from "./generic_transformation";

import { Code } from "../cells/code";

export class DeployCodeTransformation extends GenericTransformation<[], [], [Code], undefined> {
  constructor({ cellOutputs }: { cellOutputs?: [Code] }) {
    super({
      cellDeps: [],
      cellInputs: [],
      cellOutputs,
      witness: undefined,
    });
  }
}
