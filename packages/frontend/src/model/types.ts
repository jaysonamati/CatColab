import { uuidv7 } from "uuidv7";

import { DblModel } from "catlog-wasm";
import type {
    DblTheory,
    InvalidDiscreteDblModel,
    MorDecl,
    MorType,
    ObDecl,
    ObType,
    Uuid,
} from "catlog-wasm";
import { indexArray } from "../util/indexing";

/** A judgment in the definition of a model.

TODO: Judgments should be declarations *or* morphism equations.
 */
export type ModelJudgment = ModelDecl;

export type ModelDecl = ObjectDecl | MorphismDecl;

/** Declaration of an object in a model.
 */
export type ObjectDecl = ObDecl & {
    tag: "object";

    /** Human-readable name of object. */
    name: string;
};

export const newObjectDecl = (obType: ObType): ObjectDecl => ({
    tag: "object",
    id: uuidv7(),
    name: "",
    obType,
});

/** Declaration of a morphim in a model.
 */
export type MorphismDecl = MorDecl & {
    tag: "morphism";

    /** Human-readable name of morphism. */
    name: string;
};

export const newMorphismDecl = (morType: MorType): MorphismDecl => ({
    tag: "morphism",
    id: uuidv7(),
    name: "",
    morType,
    dom: null,
    cod: null,
});

/** Construct a `catlog` model from a sequence of model judgments.
 */
export function catlogModel(theory: DblTheory, judgments: Array<ModelJudgment>): DblModel {
    const model = new DblModel(theory);
    for (const judgment of judgments) {
        if (judgment.tag === "object") {
            model.addOb(judgment);
        } else if (judgment.tag === "morphism") {
            model.addMor(judgment);
        }
    }
    return model;
}

/** Result of validating a model in the categorical core. */
export type ModelValidationResult = ValidatedModel | ModelValidationErrors;

/** A valid model as represented in `catlog`. */
export type ValidatedModel = {
    tag: "validated";
    model: DblModel;
};

/** Errors in a model that did not validate. */
export type ModelValidationErrors = {
    tag: "errors";
    model: DblModel;
    errors: Map<Uuid, InvalidDiscreteDblModel<Uuid>[]>;
};

/** Validate a model in the categorical core. */
export function validateModel(
    theory: DblTheory,
    judgments: Array<ModelJudgment>,
): ModelValidationResult | undefined {
    if (theory.kind !== "Discrete") {
        // TODO: Validation should be implemented for all kinds of theories.
        return undefined;
    }
    const model = catlogModel(theory, judgments);
    const errs: InvalidDiscreteDblModel<Uuid>[] = model.validate();
    if (errs.length === 0) {
        return { tag: "validated", model };
    }
    return {
        tag: "errors",
        model,
        errors: indexArray(errs, (err) => err.content),
    };
}
