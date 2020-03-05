function getUrlByLocale(locale) {
    var defaultUrl = 'http://www.gamned.com/vie-privee/';

    var urls = {
        'fr-fr': "http://www.gamned.com/vieprivee/",
        'en-us': "http://www.gamned.com/vieprivee/",
        'en-gb': "http://www.gamned.com/vieprivee/",
        'pt-br': "http://www.gamned.com/vieprivee/"
        };

    return urls[locale] == undefined ? defaultUrl : urls[locale];
}

if (typeof g_locale == 'undefined') var g_locale = 'en-gb';
if(!urlImg) var urlImg = staticUrl + 'dco/img/mkz-infos.png';
if(!urlGam) var urlGam = getUrlByLocale(g_locale);
if(!queryGam) var queryGam = null;
if(!positionAdchoices) var positionAdchoices = 'topright';

function affDiv(display){
    document.getElementById('information_adchoices_link').style.display = display;
}

function getAdchoicesUrl(urlGam,queryGam){
   var url = getUrlByLocale(g_locale);
   if(queryGam && queryGam!='') url = url+'?'+queryGam;
   return url;
}

function changeAdchoicesUrl(urlGam,queryGam){
   document.getElementById('information_adchoices_link').href = getAdchoicesUrl(urlGam,queryGam);
}

function goGamned(){
    window.open(getAdchoicesUrl(urlGam,queryGam));
    return false;
}

var s = document.createElement('link');
s.type = 'text/css'; s.rel = 'stylesheet'; s.href = staticUrl+'dco/ad-choices.css';
s.type = 'text/css'; s.rel = 'stylesheet'; s.href = staticUrl+'dco/mkz-infos.css';
document.getElementsByTagName("head")[0].appendChild(s);

var body = document.getElementsByTagName("body")[0];
var ia = document.createElement('div');
ia.innerHTML =  '<a href="#" onClick="javascript:return goGamned();"><span id="information_adchoices_link">AdChoices</span> <img src="'+urlImg+'" height="15" alt="AdChoices" style="float: left;" /></span>'; 
ia.className = positionAdchoices; ia.id = 'information_adchoices'; ia.onmouseover = function(){affDiv('block');}; ia.onmouseout = function(){affDiv('none');}; 
body.insertBefore(ia,body.childNodes[0]);