import { useNavigate, useParams } from "@solidjs/router";
import { Index, JSX, Match, Suspense, Switch, createEffect, createResource, createSignal, onCleanup, useContext, useTransition } from "solid-js";
import invariant from "tiny-invariant";

import { type Document, useApi } from "../api";
import { FormGroup, TextAreaField, TextInputField } from "../components";

import { BrandedToolbar } from "../page";
import { AutoLabModelContext, TheoryLibraryContext } from "../stdlib";
import { ModelDocument, createModel, getLiveModel } from "../model/document";
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

    const [modelList, setModelList] = createSignal([]);

    const [jsonToRender, setJsonToRender] = createSignal("");
    const [applicationWish, setApplicationWish] = createSignal("");
    const [combinedModelReady, setCombinedModelReady] = createSignal(false);

    const [modelToTranslate, setModelToTranslate] = createSignal("");
    const [tgtLanguage, setTgtLanguage] = createSignal("");
    const [translatedModel, setTranslatedModel] = createSignal("");
    const [translatedModelReady, setTranslatedModelReady] = createSignal(false);

    const [activeModelThreads, setActiveModelThreads] = createSignal([]);
    const [selectedThread, setSelectedThread] = createSignal("");
    const [threadComposedModel, setThreadComposedModel] = createSignal("");

    const naiveJSONFromText = (text: string) => {
        const match = text.match(/\{[\s\S]*\}/);
        if (!match) return null;

        try {
            return match[0];
        } catch {
            return null;
        }
    };

    const validateAndImport = (jsonString: string) => {

        const jsonStringNaive = naiveJSONFromText(jsonString);
        if (jsonStringNaive === null) {
            return
        }
        console.log(jsonStringNaive)
        try {
            const data = JSON.parse(jsonStringNaive);

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

            // Clear paste area after successful import
            setJsonToRender("");
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
            // generateCombinedModel(applicationWish())
        }, 30000);

        onCleanup(() => {
            clearInterval(intervalId);
        });
    });

    createEffect(() => {
        // let intervalId: number;
        const intervalId = setInterval(() => {
            console.log("Generating Active Model threads");
            generateActiveModelThreads();
        }, 20000);

        onCleanup(() => {
            clearInterval(intervalId);
        });
    });


    // Define an async function to send composition POST request to our api
    const generateCombinedModel = async (application: String) => {
        console.log("Generating combined application model")
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
                setCombinedModelReady(true)

            } else {
                console.error(data.error)
                // setCombinedModel(data.error)
            }

            // Catches any errors that occur during the fetch request
        } catch (error) {
            console.error('Error:', error)
        }
    }

    // Define an async function to send translation POST request to our api
    const translateSelectedModel = async () => {
        const src_model = modelToTranslate();
        const tgt_theory = tgtLanguage();
        try {
            // use the fetch method to send an http request to /llm/composition endpoint
            const response = await fetch('http://localhost:8000/translation', {
                // mode: 'no-cors',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json; charset=utf-8',
                    // 'Access-Control-Allow-Origin':'*',
                    // 'Access-Control-Allow-Methods':'GET, POST, PUT, DELETE, OPTIONS'
                },
                body: JSON.stringify({
                    src_json_model: src_model,
                    tgt_json_language: tgt_theory,
                })
            });

            // Waits for the response to be converted to JSON format and stores it in the data variable
            const data = await response.json();

            // If successful, updates the output state with the output field from the response data
            if (response.ok) {
                setTranslatedModel(data.translated_json);
                console.log(data.translated_json);
                setTranslatedModelReady(true);

            } else {
                console.error(data.error)
                // setCombinedModel(data.error)
            }

            // Catches any errors that occur during the fetch request
        } catch (error) {
            console.error('Error:', error)
        }
    }


    // Define an async function to send translation POST request to our api
    const generateActiveModelThreads = async () => {
        const active_model = modelToTranslate();
        try {
            // use the fetch method to send an http request to /llm/composition endpoint
            const response = await fetch('http://localhost:8000/thread_suggestion', {
                // mode: 'no-cors',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json; charset=utf-8',
                    // 'Access-Control-Allow-Origin':'*',
                    // 'Access-Control-Allow-Methods':'GET, POST, PUT, DELETE, OPTIONS'
                },
                body: JSON.stringify({
                    lib_models: autolabModels.autoModels,
                    current_model: active_model,
                    thread_primer: "A very good suggestion"
                })
            });

            // Waits for the response to be converted to JSON format and stores it in the data variable
            const data = await response.json();

            // If successful, updates the output state with the output field from the response data
            if (response.ok) {
                setActiveModelThreads(data.suggested_threads)
                console.log(data.suggested_threads);
                //validateAndImport(translatedModel())

            } else {
                console.error(data.error)
                // setCombinedModel(data.error)
            }

            // Catches any errors that occur during the fetch request
        } catch (error) {
            console.error('Error:', error)
        }
    }



    // Define an async function to send translation POST request to our api
    const composeActiveModelWithSelectedThread = async () => {
        const src_model = modelToTranslate();
        const selected_thread = selectedThread();
        try {
            // use the fetch method to send an http request to /llm/composition endpoint
            const response = await fetch('http://localhost:8000/thread_composition', {
                // mode: 'no-cors',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json; charset=utf-8',
                    // 'Access-Control-Allow-Origin':'*',
                    // 'Access-Control-Allow-Methods':'GET, POST, PUT, DELETE, OPTIONS'
                },
                body: JSON.stringify({
                    active_model: src_model,
                    selected_thread: selected_thread,
                })
            });

            // Waits for the response to be converted to JSON format and stores it in the data variable
            const data = await response.json();

            // If successful, updates the output state with the output field from the response data
            if (response.ok) {
                setThreadComposedModel(data.composition_text)
                console.log(data.composition_text);
                //validateAndImport(translatedModel())

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

    const handleTargetLanguageInput: JSX.EventHandler<HTMLInputElement, Event> = (event) => {
        const textarea = event.currentTarget;
        setTgtLanguage(textarea.value);
    };

    const handleWishModelComposition = () => {
        validateAndImport(combinedModel())
        setCombinedModelReady(false)
    }

    const handleModelTranslationSelect = (model: string) => {
        setModelToTranslate(model);
        setTgtLanguage("causal-loop-delays")
        console.log("Will translate this model", model);
    }

    const handleModelTranslation = () => {
        console.log("Translation!!");
        if ((modelToTranslate() === "") || (tgtLanguage() === "")) {
            console.log("Missing data!!");
            return;
        }
        console.log("Src Model", modelToTranslate());
        console.log("Tgt language", tgtLanguage());
        translateSelectedModel();
        console.log("Translation Complete");

    }

    const handleViewTranslatedModel = () => {

        validateAndImport(translatedModel())
        console.log("Viewing Translated Model");
        setTranslatedModelReady(false);

    }

    const handleSelectedThread = (thread: string) => {
        console.log("Selected Thread", thread);
        setSelectedThread(thread);
    }


    const handleSelectedThreadComposition = (thread: string) => {
        console.log("Selected Thread", thread);
        if (selectedThread() === "") {
            console.log("No Thread selected")
            return;
        }
        composeActiveModelWithSelectedThread();
    }



    const [_liveModel] = createResource(
        () => params.ref,
        (refId) => getLiveModel(refId, api, theories),
    );

    // Logic and state for tabs
    const [tab, setTab] = createSignal(0);
    const [pending, start] = useTransition();
    const updateTab = (index: number) => () => start(() => setTab(index));

    return (
        <>
            <div class="autocatlab-container">
                <BrandedToolbar />
                <h1 class="auto-title">AutoLab</h1>
                <div class="model-list">
                    <Index each={autolabModels.autoModels} fallback={<div>Loading...</div>}>
                        {(item, _index) => (
                            <div class="model-list-item" onClick={() => handleModelTranslationSelect(item())}>
                                {JSON.parse(item()).name}
                            </div>
                        )}
                    </Index>
                </div>
                <ul class="inline">
                    <li classList={{ selected: tab() === 0 }} onClick={updateTab(0)}>
                        Composition
                    </li>
                    <li classList={{ selected: tab() === 1 }} onClick={updateTab(1)}>
                        Transform
                    </li>
                    <li classList={{ selected: tab() === 2 }} onClick={updateTab(2)}>
                        Threads
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
                                        <button type="button" class="ok" onClick={() => generateCombinedModel(applicationWish())}>
                                            Suggest Combined Model
                                        </button>
                                        <button type="button" class="ok disabled" disabled={!combinedModelReady()} onClick={() => handleWishModelComposition()}>
                                            View Combined Model
                                        </button>

                                    </FormGroup>
                                </form>
                            </Match>
                            <Match when={tab() === 1}>
                                <h1>Transformation</h1>
                                <form>
                                    <FormGroup>
                                        <TextInputField
                                            label="Your Target Language"
                                            value={tgtLanguage()}
                                            onInput={handleTargetLanguageInput}
                                            onPaste={handleTargetLanguageInput}
                                            placeholder="causal-loop"
                                        />
                                        <button type="button" class="ok" onClick={() => handleModelTranslation()}>
                                            Translate Selected Model To Personal Language
                                        </button>
                                        <button type="button" class="ok" disabled={!translatedModelReady()} onClick={() => handleViewTranslatedModel()}>
                                            View Translated Model
                                        </button>
                                    </FormGroup>
                                </form>
                            </Match>
                            <Match when={tab() === 2}>
                                <div class="model-list">
                                    <Index each={activeModelThreads()} fallback={<div>Loading...</div>}>
                                        {(thread, _index) => (
                                            <div onClick={() => handleSelectedThread(thread())}>
                                                {thread()}
                                            </div>
                                        )}
                                    </Index>
                                </div>
                                <h1>Threads</h1>
                                <button type="button" class="ok" onClick={() => handleSelectedThreadComposition(selectedThread())}>
                                    Compose Active Model with Selected Thread Enhancements
                                </button>
                            </Match>
                        </Switch>
                    </Suspense>

                </div>
            </div>
        </>
    );
}
