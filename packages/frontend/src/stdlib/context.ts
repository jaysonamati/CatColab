import { createContext } from "solid-js";

import { AutolabModelLibrary, type TheoryLibrary } from "./types";

/** Context for the active library of double theories. */
export const TheoryLibraryContext = createContext<TheoryLibrary>();

export const AutoLabModelLibraryContext = createContext<AutolabModelLibrary>();

export const AutoLabModelContext = createContext();
