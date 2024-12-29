import { Session, retrieveCasUrl } from "scodok";

const session = new Session("https://notes9.iutlan.univ-rennes1.fr");
const url = await retrieveCasUrl(session);
console.log(url);
