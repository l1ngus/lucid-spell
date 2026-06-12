import { LangPair } from "../types/Langs";
import { createDictionary, addPair, removePair } from "./dictionariesStore";


export async function ensureFavoriteDictionary() { }
export async function addFavoriteTranslation(source: string, target: string, langPair: LangPair) { }
export async function isTranslationFavorite(source: string, target: string, langPair: LangPair) { }
export async function removeFavoriteTranslationByContent(source: string, target: string, langPair: LangPair) { }
