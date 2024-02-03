import { FormLiteral } from "@/components/LiteralList";

export type Language = {
    code: string;
    default: boolean;
};

export type TranslationPage = {[key: string]: string};

export default (baseUrl: string) => {
    return {
        getPages: async () : Promise<string[]> => {
            "use server";
            return await fetch(
                `${baseUrl}/pages`, 
                { 
                    cache: 'no-store',
                    method: "GET" 
            }).then(res => res.json());
        },

        createPage: async (id: string) : Promise<boolean> => {
            "use server";
            return fetch(
                `${baseUrl}/pages`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({
                        id
                    }),
                }
            ).then(response => response.ok);
        },

        getPage: async (id: string, language: string) : Promise<TranslationPage> => {
            "use server";
            return await fetch(
                `${baseUrl}/${id}/${language}`,
                { 
                    cache: 'no-store',
                    method: "GET" 
            }).then(res => res.json());
        },

        saveLiterals: async (id: string, language: string, literals: FormLiteral[]) : Promise<boolean> => {
            "use server";
            return fetch(
                `${baseUrl}/${id}/${language}`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(literals),
                }
            ).then(response => response.ok);
        },

        getLanguages: async () : Promise<Language[]> => {
            "use server";
            return await fetch(
                `${baseUrl}/languages`, 
                { 
                    cache: 'no-store',
                    method: "GET" 
            }).then(res => res.json());
        },

        saveLanguages: async (languages: Language[]) : Promise<boolean> => {
            "use server";
            return await fetch(
                `${baseUrl}/languages`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(languages),
                }
            ).then(response => response.ok);
        }
    }
}