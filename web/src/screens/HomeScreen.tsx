import { Link } from "react-router";
import { fr } from "../i18n/fr";

export function HomeScreen() {
  return (
    <main className="centered">
      <h1 className="title">{fr.home.greeting}</h1>
      <p className="message">{fr.home.prompt}</p>
      <Link className="button button-primary" to="/addition">
        {fr.home.startAddition}
      </Link>
    </main>
  );
}
