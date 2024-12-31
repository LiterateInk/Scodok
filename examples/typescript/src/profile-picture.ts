import { Session, getProfilePictureBytes } from "scodok";
import fs from "node:fs/promises";

const session = new Session("https://notes.scodoc.local/", "YOUR_PHPSESSID");
const bytes = await getProfilePictureBytes(session);
console.log("Received", bytes.length, "bytes of picture.");

// Let's save the picture to a file !
await fs.writeFile("profile-picture.jpg", bytes);
