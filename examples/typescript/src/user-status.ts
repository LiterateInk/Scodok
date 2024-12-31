import { Session, UserStatus, getUserStatus } from "scodok";

const session = new Session("https://notes.scodoc.local/", "YOUR_PHPSESSID");

const userEmail = "someone@univ-example.fr"; // Grab this from any way you want...
const status = await getUserStatus(session, userEmail);
console.log(`Status of the user '${userEmail}' is ${UserStatus[status]}`);
