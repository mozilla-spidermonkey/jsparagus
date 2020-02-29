/** Generates 32 hex digits guid grouped into chunks of 8-4-4-4-12
 * implemented following guidelines of rfc4122 from ietf
 */
function generateGuid() {
    var intArray = new Uint32Array(6);
    window.crypto.getRandomValues(intArray);
    var guid = intArray[0].toString(16)
    + '-'
    + intArray[1].toString(16).substring(0,4)
    + '-'
    + intArray[2].toString(16).substring(0,4)
    + '-'
    + intArray[3].toString(16).substring(0,4)
    + '-'
    + intArray[4].toString(16)
    + intArray[5].toString(16).substring(0,4);

    return guid;
}
/** If cookie is present reuse existing guid
 *  else create a new guid and set the cookie
 */
function getGuid () {
    var cookieName = 'MPPGuid';
    var oldCookie = new mi.Cookie(document, cookieName);
    oldCookie.load();
    var guid = '';

    if( oldCookie['guid'] ) {
        guid = oldCookie['guid'];
    }
    else {
        var expireInMinutes = Math.round((new Date('Thu, 1 Jan 2099 10:00:00 GMT').getTime() - new Date().getTime()) / 60000);
        var guidCookie = new mi.Cookie(document, cookieName, expireInMinutes, '/');
        //new mi.Cookie(document, name, minutes, path, domain, secure);
        guid = generateGuid();
        guidCookie['guid'] = guid;
        guidCookie.store();
    }

    return guid;
}