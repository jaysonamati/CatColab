import { useParams } from "@solidjs/router";
import { Match, Show, Switch, createResource, useContext } from "solid-js";
import invariant from "tiny-invariant";

import { useApi } from "../api";
import { InlineInput } from "../components";
import {
    type CellConstructor,
    type FormalCellEditorProps,
    NotebookEditor,
    cellShortcutModifier,
    newFormalCell,
} from "../notebook";
import { DocumentMenu, TheoryHelpButton, Toolbar } from "../page";
import { TheoryLibraryContext } from "../stdlib";
import type { ModelTypeMeta } from "../theory";
import { MaybePermissionsButton } from "../user";
import { LiveModelContext } from "../model/context";
import { type LiveModelDocument, getLiveModel } from "../model/document";
import { MorphismCellEditor } from "../model/morphism_cell_editor";
import { ObjectCellEditor } from "../model/object_cell_editor";
import { TheorySelectorDialog } from "../model/theory_selector";
import {
    type ModelJudgment,
    type MorphismDecl,
    type ObjectDecl,
    duplicateModelJudgment,
    newMorphismDecl,
    newObjectDecl,
} from "../model/types";

import "./model_editor.css";

export default function ModelPage() {
    const api = useApi();
    const theories = useContext(TheoryLibraryContext);
    invariant(theories, "Must provide theory library as context to model page");

    const params = useParams();

    const [liveModel] = createResource(
        () => params.ref,
        (refId) => getLiveModel(refId, api, theories),
    );

    return <ModelDocumentEditor liveModel={liveModel()} />;
}