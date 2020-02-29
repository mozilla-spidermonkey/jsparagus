
var language = window.navigator.userLanguage || window.navigator.language;
language = language.toUpperCase().substr(0, 2);

if (images[language] === undefined) {
    language = 'EN';
}


var image = images[language][Math.floor(Math.random() * images[language].length)];



var paramsMapping = {
    'custom1': 'CampaignID',
    'custom2': 'SiteName',
    'custom3': 'Location',
    'custom4': 'BanID',
    'custom5': 'SpotID',
    'custom6': 'BidID',
    'custom7': 'BidValue',
    'custom10': 'ntk'
    
};


var linktogo = getURLParameter('linktogo');
var Postbacktogo = "na";
var baseLink = linktogo;
var basePostback = Postbacktogo;
var link = baseLink;
var urlParams = "";
var postback = "";




// Choose a random image
var urlParams = getJsonFromUrl();
var baseLink = "https://offaces-butional.com/" + urlParams.custom1;
var basePostback = "https://offaces-butional.com/impression/" + urlParams.custom1;
var link = buildUrl(baseLink, urlParams, paramsMapping);
link += "&BanID" + "=" + baseName(image);
var postback = buildUrl(basePostback, urlParams, paramsMapping);
postback += "&BanID" + "=" + baseName(image);

document.getElementById('link').setAttribute('href', link);
document.getElementById('image').setAttribute('src', image);
document.getElementById('postback').setAttribute('src', postback);


// Choose a random image
document.getElementById('link').setAttribute('href', link);
document.getElementById('image').setAttribute('src', image);
document.getElementById('postback').setAttribute('src', postback);



// Setup baseLink & basePostback
function getURLParameter(name, url) {
    if (!url) url = window.location.href;
    name = name.replace(/[\[\]]/g, "\\$&");
    name = name.replace(/[\[\]]/g, "\\$&");
    var regex = new RegExp("[?&]" + name + "(=([^&#]*)|&|#|$)"),
        results = regex.exec(url);
    if (!results) return null;
    if (!results[2]) return '';
    return decodeURIComponent(results[2].replace(/\+/g, " "));
}

function getJsonFromUrl() {
    var result = {};
    var query = location.search.substr(1);
    if (query.length === 0) {
        return result;
    }
    query.split("&").forEach(function(part) {
        var item = part.split("=");
        result[item[0]] = decodeURIComponent(item[1]);
    });
    return result;
}

function buildUrl(baseUrl, urlParams, mapping) {
    for (var key in mapping) {
        if (!mapping.hasOwnProperty(key)) {
            continue;
        }
        if (urlParams[key] != undefined) {
            if (baseUrl.indexOf('?') === -1) {
                baseUrl += '?';
            } else {
                baseUrl += '&';
            }
            baseUrl += encodeURI(mapping[key]) + "=" + encodeURI(urlParams[key]);
        }
    }
    return baseUrl;
}

function baseName(str) {
    var base = str.substring(str.lastIndexOf('/') + 1);
    if (base.lastIndexOf(".") !== -1) {
        base = base.substring(0, base.lastIndexOf("."));
    }
    return base;
}