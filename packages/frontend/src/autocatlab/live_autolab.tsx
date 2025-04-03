import { useNavigate, useParams } from "@solidjs/router";
import { JSX, Match, Show, Suspense, Switch, createEffect, createResource, createSignal, onCleanup, useContext, useTransition } from "solid-js";
import invariant from "tiny-invariant";

import { type Document, useApi } from "../api";
import { FormGroup, InlineInput, TextAreaField } from "../components";
import {
    type CellConstructor,
    type FormalCellEditorProps,
    NotebookEditor,
    cellShortcutModifier,
    newFormalCell,
} from "../notebook";
import { AutoCatlabButton, BrandedToolbar, DefaultAppMenu, DocumentMenu, TheoryHelpButton, Toolbar } from "../page";
import { AutoLabModelContext, TheoryLibraryContext } from "../stdlib";
import type { ModelTypeMeta } from "../theory";
import { MaybePermissionsButton } from "../user";
import { LiveModelContext } from "../model/context";
import { type LiveModelDocument, ModelDocument, createModel, getLiveModel } from "../model/document";
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

import "./live_autolab.css";
import { createDiagramFromDocument, DiagramDocument } from "../diagram";



type ImportableDocument = ModelDocument | DiagramDocument;

function isImportableDocument(doc: Document<string>): doc is ImportableDocument {
    return doc.type === "model" || doc.type === "diagram";
}


export default function AutoModelPage() {
    const api = useApi();
    const navigate = useNavigate();
    const theories = useContext(TheoryLibraryContext);
    invariant(theories, "Must provide theory library as context to model page");

    const { autolabModels, setAutoLabModels } = useContext(AutoLabModelContext);

    const [jsonToRender, setJsonToRender] = createSignal("");
    const [applicationWish, setApplicationWish] = createSignal("");

    const validateAndImport = (jsonString: string) => {
        try {
            const data = JSON.parse(jsonString);

            // Run custom validation if provided
            // if (validate) {
            //     const validationResult = validate(data);
            //     if (typeof validationResult === "string") {
            //         setError(validationResult);
            //         return;
            //     }
            // }

            // Clear any previous errors and import
            // setError(null);
            handleLLMJson(data);
            setJsonToRender(""); // Clear paste area after successful import
        } catch (e) {
            console.log(e);
        }
    };




    const handleLLMJson = async (data: Document<string>) => {
        invariant(
            isImportableDocument(data),
            "Analysis and other document types cannot be imported at this time.",
        );

        switch (data.type) {
            case "model": {
                const newRef = await createModel(api, data);
                try {
                    navigate(`/model/${newRef}`);
                } catch (e) {
                    throw new Error(`Failed to navigate to new ${data.type}: ${e}`);
                }
                break;
            }

            case "diagram": {
                const newRef = await createDiagramFromDocument(api, data);
                try {
                    navigate(`/diagram/${newRef}`);
                } catch (e) {
                    throw new Error(`Failed to navigate to new ${data.type}: ${e}`);
                }
                break;
            }

            default:
                throw new Error("Unknown document type");
        }

        // props.onComplete?.();
    };



    const params = useParams();

    // This takes the output of the LLM (json) and renders it in a document, the same way we import the json of a model.
    const [combinedModel, setCombinedModel] = createSignal("");

    createEffect(() => {
        console.log("The response from LLM is", combinedModel);
    });

    //https://playground.solidjs.com/anonymous/cc0510eb-7ea2-415f-b808-617bb6b96e7a
    createEffect(() => {
        // let intervalId: number;
        const intervalId = setInterval(() => {
            console.log("Communicating with AI");
            console.log("The models in store", autolabModels.autoModels);
            //generateCombinedModel(applicationWish())
        }, 5000);

        onCleanup(() => {
            clearInterval(intervalId);
        });
    });


    // Define an async function to send POST request to our api
    const generateCombinedModel = async (application: String) => {
        try {
            // use the fetch method to send an http request to /llm/composition endpoint
            const response = await fetch('http://localhost:8000/composition', {
                // mode: 'no-cors',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json; charset=utf-8',
                    // 'Access-Control-Allow-Origin':'*',
                    // 'Access-Control-Allow-Methods':'GET, POST, PUT, DELETE, OPTIONS'
                },
                body: JSON.stringify({
                    models: autolabModels.autoModels,
                    application: application
                })
            });

            // Waits for the response to be converted to JSON format and stores it in the data variable
            const data = await response.json();

            // If successful, updates the output state with the output field from the response data
            if (response.ok) {
                setCombinedModel(data.composition_text)
                console.log(data.composition_text)

            } else {
                console.error(data.error)
                // setCombinedModel(data.error)
            }

            // Catches any errors that occur during the fetch request
        } catch (error) {
            console.error('Error:', error)
        }
    }


    const handleWishInput: JSX.EventHandler<HTMLTextAreaElement, Event> = (event) => {
        const textarea = event.currentTarget;
        setApplicationWish(textarea.value);
    };

    const handleWishModelComposition = () => {
        validateAndImport(combinedModel())
    }



    const [liveModel] = createResource(
        () => params.ref,
        (refId) => getLiveModel(refId, api, theories),
    );

    // Logic and state for tabs
    const [tab, setTab] = createSignal(0);
    const [pending, start] = useTransition();
    const updateTab = (index) => () => start(() => setTab(index));

    return (
        <>
            <div class="autocatlab-container">
                {/* <ModelDocumentEditor liveModel={liveModel()} /> */}
                <BrandedToolbar />
                {/* <Toolbar>
                    <DefaultAppMenu />
                    <span class="filler" />
                    <AutoCatlabButton/>
                    <Brand/>
                </Toolbar> */}
                <h1 class="auto-title">AutoLab</h1>
                <ul class="inline">
                    <li classList={{ selected: tab() === 0 }} onClick={updateTab(0)}>
                        Composition
                    </li>
                    <li classList={{ selected: tab() === 1 }} onClick={updateTab(1)}>
                        Transform
                    </li>
                </ul>
                <div class="tab" classList={{ pending: pending() }}>
                    <Suspense fallback={<div class="loader">Loading...</div>}>
                        <Switch>
                            <Match when={tab() === 0}>
                                <form>
                                    <FormGroup>
                                        <TextAreaField
                                            label="Application wish"
                                            value={applicationWish()}
                                            onInput={handleWishInput}
                                            onPaste={handleWishInput}
                                            placeholder="Make an application wish"
                                        />
                                        <button type="button" class="ok" onClick={() => handleWishModelComposition()}>
                                            Suggest Combined Model
                                        </button>

                                    </FormGroup>
                                </form>
                            </Match>
                            <Match when={tab() === 1}>
                                <h1>Transformation</h1>
                            </Match>
                        </Switch>
                    </Suspense>

                </div>
            </div>
        </>
    );
}
