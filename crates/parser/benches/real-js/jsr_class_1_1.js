// JSONscriptRequest -- a simple class for accessing Yahoo! Web Services

// using dynamically generated script tags and JSON

//

// Author: Jason Levitt

// Date: December 7th, 2005

//

// A SECURITY WARNING FROM DOUGLAS CROCKFORD:

// "The dynamic <script> tag hack suffers from a problem. It allows a page 

// to access data from any server in the web, which is really useful. 

// Unfortunately, the data is returned in the form of a script. That script 

// can deliver the data, but it runs with the same authority as scripts on 

// the base page, so it is able to steal cookies or misuse the authorization 

// of the user with the server. A rogue script can do destructive things to 

// the relationship between the user and the base server."

//

// So, be extremely cautious in your use of this script.

//



// Constructor -- pass a REST request URL to the constructor

//

function JSONscriptRequest(fullUrl) {

    // REST request path

    this.fullUrl = fullUrl; 

    // Keep IE from caching requests

    this.noCacheIE = '&noCacheIE=' + (new Date()).getTime();

    // Get the DOM location to put the script tag

    this.headLoc = document.getElementsByTagName("head").item(0);

    // Generate a unique script tag id

    this.scriptId = 'JscriptId' + JSONscriptRequest.scriptCounter++;

}



// Static script ID counter

JSONscriptRequest.scriptCounter = 1;



// buildScriptTag method

//

JSONscriptRequest.prototype.buildScriptTag = function () {



    // Create the script tag

    this.scriptObj = document.createElement("script");

    

    // Add script object attributes

    this.scriptObj.setAttribute("type", "text/javascript");
    this.scriptObj.setAttribute("charset", "utf-8");

    this.scriptObj.setAttribute("src", this.fullUrl + this.noCacheIE);

    this.scriptObj.setAttribute("id", this.scriptId);

}

 

// removeScriptTag method

// 

JSONscriptRequest.prototype.removeScriptTag = function () {

    // Destroy the script tag

    this.headLoc.removeChild(this.scriptObj);  

}



// addScriptTag method

//

JSONscriptRequest.prototype.addScriptTag = function () {

    // Create the script tag

    this.headLoc.appendChild(this.scriptObj);

}



// this function is Yahoo! News original function
JSONscriptRequest.prototype.setCache = function () {
    this.noCacheIE = '';
}

