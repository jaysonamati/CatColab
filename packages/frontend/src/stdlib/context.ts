import { createContext } from "solid-js";

import type { AutolabModelLibrary, TheoryLibrary } from "./types";

/** Context for the active library of double theories. */
export const TheoryLibraryContext = createContext<TheoryLibrary>();

/** Context for the user's models added to live library. */

export const AutoLabModelContext = createContext();

/** More comprehensive live model library. */
export const AutoLabModelLibraryContext = createContext<AutolabModelLibrary>();

