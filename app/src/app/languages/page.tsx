import { useServerClient } from "@/client";
import { Language } from "@/client/TranslationClient";
import LanguagesLayout from "@/components/LanguagesLayout";


export default async () => {
    const client = useServerClient();
    const languages = await client.getLanguages();

    async function saveLanguage(languages: Language[]) {
        "use server";
        return await client.saveLanguages(languages);
    }

    return <LanguagesLayout 
        languages={languages}
        onSaveChanges={saveLanguage}/>;
}