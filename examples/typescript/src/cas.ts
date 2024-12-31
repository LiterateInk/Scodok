import { getCasUrl, getSessionFromCasTicket } from "scodok";

const instanceUrl = "https://notes.scodoc.local/";
const casUrl = await getCasUrl(instanceUrl);
console.log("You have to authenticate to your CAS at the following URL:", casUrl);

// Let's say we grabbed this variable from the CAS authentication...
const ticket = "ST-123456789-SOMELONGSTRING-vmjava-pcas2";
const session = await getSessionFromCasTicket(instanceUrl, ticket);
console.log("You are now authenticated in ScoDoc Notes with the following PHP session ID:", session.phpSessId);
