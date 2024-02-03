import TranslationClient from "./TranslationClient";

export default () => {
    const baseUrl = process.env.API_URL || "";
    return TranslationClient(baseUrl);
};