import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";

import styles from "./page.module.scss";
import PageList from "@/components/PageList";
import useServerClient from "@/client/useServerClient";
import { Alert } from "react-bootstrap";
import Link from "next/link";
import LiteralList, { FormLiteral } from "@/components/LiteralList";

export default async function Home({
  params,
  searchParams,
}: {
  params: { page: string };
  searchParams: { language: string };
}) {
  const client = useServerClient();

  const pages = await client.getPages();
  const languages = await client.getLanguages();
  const defaultLanguage = languages.find((language) => language.default);
  console.log(await client.getSortedPage(params.page, searchParams.language));
  const translation = (
    await client.getSortedPage(params.page, searchParams.language)
  ).map(({ key, text }) => ({ key, value: text }));

  let defaultTranslation = undefined;
  if (defaultLanguage && searchParams.language !== defaultLanguage.code) {
    defaultTranslation = Object.entries(
      await client.getPage(params.page, defaultLanguage.code)
    ).map(([key, value]) => ({ key, value }));
  }

  async function create(page: string) {
    "use server";
    return await client.createPage(page);
  }

  async function saveLiterals(literals: FormLiteral[]) {
    "use server";
    return await client.saveLiterals(
      params.page,
      searchParams.language,
      literals
    );
  }

  return (
    <Container fluid={defaultLanguage !== undefined}>
      {!defaultLanguage ? (
        <Alert variant="danger">Set a default language</Alert>
      ) : (
        <Row>
          <Col md={2} className={styles.sidebar}>
            <PageList
              defaultLanguage={defaultLanguage!.code}
              onCreatePage={create}
              value={pages}
            />
          </Col>

          <Col>
            <div className="my-2 px-2">
              Available languages:
              <span className="ms-2 d-inline-flex flex-wrap gap-2">
                {languages.map((language) => (
                  <Link href={`/${params.page}?language=${language.code}`}>
                    {language.code}
                  </Link>
                ))}
              </span>
            </div>

            <LiteralList
              language={searchParams.language}
              defaultTranslation={defaultTranslation}
              literals={translation}
              saveChanges={saveLiterals}
            />
          </Col>
        </Row>
      )}
    </Container>
  );
}
