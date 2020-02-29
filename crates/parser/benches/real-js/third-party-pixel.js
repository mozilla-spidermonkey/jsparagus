/**
 * Created by william on 19/01/16.
 */
function guv(url){
    if(url === undefined){
        url = window.location.href;
    }
    for (var t = [], e = url.slice(url.indexOf("?") + 1).split("&"), n = 0; n < e.length; n++){
        var i = e[n].indexOf("="), o = e[n].substring(0, i), d = e[n].substring(i + 1, e[n].length); t.push(o), t[o] = d}return t}

function buildQuery(parameters){
    var qs = "";
    for(var key in parameters) {
        if(key !== "pos" || key !== "lang"){
            continue;
        }
        var value = parameters[key];
        qs += encodeURIComponent(key) + "=" + encodeURIComponent(value) + "&";
    }
    if (qs.length > 0){
        qs = qs.substring(0, qs.length-1); //chop off last "&"
        qs =  "?" + qs;
    }
    return qs;
}

function affDiv(display){
    document.getElementById('information_adchoices_link').style.display = display;
}

function getAdchoicesUrl(urlGam,queryGam){
    if(!urlGam){
        var urlGam = aUrl['fr-fr'];
    }
    if(queryGam && queryGam !='' ) {
        urlGam = urlGam + queryGam;
    }
    return urlGam;
}

function goGamned(){
    window.open(getAdchoicesUrl(urlGam, buildQuery(queryGam)));
    return false;
}

var validParams = {
    'pos' : ['topright','topleft','bottomright','bottomleft'],
    'lang' : ['fr-fr', 'fr-ch', 'en-en', 'pt-br', 'de-ch']
};

if(typeof staticUrl  === "undefined"){
    staticUrl = '//static.adbutter.net/';
}

var aUrl = {
    'fr-fr' : 'http://www.gamned.com/vie-privee',
    'fr-ch' : 'http://www.gamned.com/ch/vie-privee',
    'de-ch' : 'http://www.gamned.com/de/datenschutz',
    'pt-br' : 'http://www.gamned.com/en/privacy',
    'en-en' : 'http://www.gamned.com/en/privacy'
};

var queryGam = {'pos':undefined, 'lang':undefined};

var lstScript = document.getElementsByTagName("script");
for (var i = 0; i< lstScript.length;i++) {

    if (lstScript[i].src.indexOf("adbutter.net") === -1){
        continue;
    }

    if (lstScript[i].src.indexOf("third-party-pixel") !== -1 && lstScript[i].src.indexOf("?") !== -1){
        var pos = lstScript[i].src.indexOf("?");

        console.log(lstScript[i].src.substr(pos+1));

        queryGam = guv(lstScript[i].src.substr(pos+1));
    }
}





if(!urlImg) var urlImg = staticUrl + 'dco/img/adchoices.png';
if(!queryGam) var queryGam = null;
if(!urlGam){
    var urlGam;
    if(queryGam.lang !== undefined){

        switch(queryGam.lang) {
            case 'fr-fr': urlGam = aUrl[queryGam.lang];
                break;
            case 'fr-ch': urlGam = aUrl[queryGam.lang];
                break;
            case 'de-ch': urlGam = aUrl[queryGam.lang];
                break;
            case 'pt-br':
            case 'en-en': urlGam = aUrl[queryGam.lang];
                break;
            default: urlGam = aUrl['fr-fr'];
                break;
        }
    }
}

if(validParams['pos'].indexOf(queryGam['pos']) !== -1)
    queryGam['pos'] =  queryGam['pos'];
else
    queryGam['pos'] = 'topright';

if(validParams['lang'].indexOf(queryGam['lang']) !== -1)
    queryGam['lang'] =  queryGam['lang'];
else
    queryGam['lang'] = 'fr-fr';

var s = document.createElement('link');
s.type = 'text/css';
s.rel = 'stylesheet';
s.href = staticUrl+'dco/ad-choices.css';
document.getElementsByTagName("head")[0].appendChild(s);

var body = document.getElementsByTagName("body")[0];
var ia = document.createElement('div');

ia.innerHTML =  '<a href="#" onClick="javascript:return goGamned();"><span id="information_adchoices_link">AdChoices</span><img src="'+urlImg+'" height="15" alt="AdChoices" style="float: left;" /></span>';

ia.className = queryGam['pos'];
ia.id = 'information_adchoices';
ia.onmouseover = function(){
    affDiv('block');
};
ia.onmouseout = function(){
    affDiv('none');
};
body.insertBefore(ia,body.childNodes[0]);
