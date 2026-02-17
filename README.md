# PATTMAYNE AUTH

The name of the site will be CRANKADE.
Like and ARCADE for CRANKS.

An authentication app I will use to serve a few web apps and games I intend to make.
I'll use JSON webtokens (JWTs) reinforced by refresh_tokens.

### TO DO:
 * Modal popup demands confirmation before generating new client_secret
 * Add developer blog
 * Terms and Conditions for signup
 * Table for site_info
 * * "singleton" style... all text... key-value
 * Email verification of accounts (with Mailjet https://www.mailjet.com/pricing/)
 * Integrate donations (only after multiple games)
 * Turn on Content-Encoding: gzip
 * Write an essay about the auth flow between instances to help remember the details.
 * JWT EXPIRED notice arrives too many times. Route or middleware being hit too many times.
 * * Depends on what page you're on.
 * Move login/register logic into auth.rs module.
 * * Don't return http stuff to the route function. Just return the data that the user needs.
 * Forgot Password option

### Client Tokens Structure:
The client apps will set JWTs as access tokens into the user's browser's secure cookies. JWTs will expire every few minutes (somewhere within an hour) and be refreshed based on user's refresh token (which is also stored in a secure cookie). JWTs are not stored on any server, only in the browser. But each client app can verify the token, and each client app has its own JWT secret.

The auth app (this app) will issue the refresh tokens and save them in the database. **Problem:** the auth app cannot set cookies for a user who is interacting with a different URL. **Solution:** when the user's refresh_token expires, the client app will use its client_secret to communicate with the auth app (this app), and the auth app will issue a new refresh_token. The client app can then set the refresh token (and a new JWT) into the user's browser's secure cookies.

Client sites are stored in a clients table in the DB.
Refresh tokens are stored in a refresh_tokens table in the DB. Refresh token entries include a user_id and a client_id. A user has a different token for each client. The expiry date of each token should be the same, to ensure that the user is made to log in periodically. When the user is logged into one client, they are logged into all. But when one refresh token expires, they all expire.

For most requests the user makes on the client site, they do NOT need to interact with the auth app (this app). Client apps have some autonomy.

### RESOURCES FILE
* French and English valies are stored in a phf::phf_map!
* * keys are all static string slice references
* Lang field in UserReqData dictates which language version to use
* Each Askama html template has a dedicated struct for all text
