import type { LangCode } from "../types/Langs";

interface GetTranslationPromptParams {
  text: string;
  sourceLang: LangCode | 'auto';
  targetLang: LangCode;
}

export const getTranslationPrompt = ({ text, sourceLang, targetLang }: GetTranslationPromptParams) => {
  return `You are an expert linguist and professional translator. Your task is to translate the provided text from ${sourceLang} to ${targetLang}, and verify the grammatical correctness of the source text.

Strictly adhere to the following rules:
1. Translate the text accurately, preserving the original meaning, tone, nuances, and formatting.
2. Analyze the source text for any spelling, grammar, or typographical errors. If the source text contains errors, provide the corrected version in the "sourceCorrection" field. If the source text is written correctly, leave the "sourceCorrection" field completely empty ("").
3. Output strictly and ONLY in valid JSON format.
4. Do not include any markdown formatting (e.g., \`\`\`json), conversational filler, explanations, or greetings.
5. Ensure any quotation marks or special characters inside the text fields are properly escaped to maintain valid JSON.

Expected JSON Output Format:
  {
    "translation": "<translated text goes here>",
      "sourceCorrection": "<corrected source text if errors exist, otherwise empty string>"
  }

Input Data:
Source Language: ${sourceLang}
Target Language: ${targetLang}
Text to translate: ${text} `
}

interface GetOtherTranslationsParams {
  sourceText: string;
  translatedText: string;
  sourceLang: LangCode | 'auto';
  targetLang: LangCode;
}

export const getOtherTranslationsPrompt = ({
  sourceText,
  translatedText,
  sourceLang,
  targetLang
}: GetOtherTranslationsParams) => {
  return `You are an expert linguist. Your task is to produce alternative translations of the source text.

Input data:
- sourceText: ${sourceText}
- translatedText: ${translatedText}
- sourceLang: ${sourceLang}
- targetLang: ${targetLang}

Rules:
1. Analyze sourceText and translatedText.
2. If sourceText is a SINGLE WORD (no spaces), return word alternatives using parts of speech:
{"otherTranslations":[{"part":"<part of speech>","translation":"<alternative translation>"}, ...]}
3. If sourceText contains spaces (phrase or sentence), return phrase/sentence alternatives without parts of speech:
{"otherTranslations":["<alternative translation 1>","<alternative translation 2>", ...]}
4. If the text cannot be translated differently (for example: proper nouns, technical terms without synonyms, filenames, numbers, abbreviations without expansion), return:
{"otherTranslations":[]}
5. Return only alternative translations in targetLang.
6. Do not add explanations, notes, markdown, or any text outside the JSON.
7. Return STRICTLY valid JSON.`;
}
