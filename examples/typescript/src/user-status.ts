import { Session, UserStatus, getUserStatus, ServerError, FetcherError } from "scodok";

const session = new Session("https://notes.scodoc.local/", "YOUR_PHPSESSID");

try {
  const userEmail = "someone@univ-example.fr";
  const status = await getUserStatus(session, userEmail);
  console.log(`Status of the user '${userEmail}' is ${UserStatus[status]}`);
}
catch (error) {
  // You can also catch exceptions !
  if (error instanceof ServerError) {
    console.error("The email you gave is not correct !\n\n");
  }
  else if (error instanceof FetcherError) {
    console.error("An error occured while fetching the data from the server.\n\n");
  }

  // Let's just throw it back to see it in the console.
  throw error;
}
