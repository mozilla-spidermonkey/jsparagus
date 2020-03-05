(function(){var k,l=this;function m(a){return"string"==typeof a}
function n(a){a=a.split(".");for(var b=l,c=0;c<a.length;c++)if(b=b[a[c]],null==b)return null;return b}
function aa(){}
function q(a){var b=typeof a;if("object"==b)if(a){if(a instanceof Array)return"array";if(a instanceof Object)return b;var c=Object.prototype.toString.call(a);if("[object Window]"==c)return"object";if("[object Array]"==c||"number"==typeof a.length&&"undefined"!=typeof a.splice&&"undefined"!=typeof a.propertyIsEnumerable&&!a.propertyIsEnumerable("splice"))return"array";if("[object Function]"==c||"undefined"!=typeof a.call&&"undefined"!=typeof a.propertyIsEnumerable&&!a.propertyIsEnumerable("call"))return"function"}else return"null";
else if("function"==b&&"undefined"==typeof a.call)return"object";return b}
function t(a){var b=typeof a;return"object"==b&&null!=a||"function"==b}
var v="closure_uid_"+(1E9*Math.random()>>>0),w=0;function ba(a,b,c){return a.call.apply(a.bind,arguments)}
function ca(a,b,c){if(!a)throw Error();if(2<arguments.length){var d=Array.prototype.slice.call(arguments,2);return function(){var e=Array.prototype.slice.call(arguments);Array.prototype.unshift.apply(e,d);return a.apply(b,e)}}return function(){return a.apply(b,arguments)}}
function x(a,b,c){Function.prototype.bind&&-1!=Function.prototype.bind.toString().indexOf("native code")?x=ba:x=ca;return x.apply(null,arguments)}
var da=Date.now||function(){return+new Date};
function y(a,b){var c=a.split("."),d=l;c[0]in d||"undefined"==typeof d.execScript||d.execScript("var "+c[0]);for(var e;c.length&&(e=c.shift());)c.length||void 0===b?d[e]&&d[e]!==Object.prototype[e]?d=d[e]:d=d[e]={}:d[e]=b}
function z(a,b){function c(){}
c.prototype=b.prototype;a.J=b.prototype;a.prototype=new c;a.prototype.constructor=a;a.L=function(d,e,f){for(var g=Array(arguments.length-2),h=2;h<arguments.length;h++)g[h-2]=arguments[h];return b.prototype[e].apply(d,g)}}
;var C=Array.prototype.indexOf?function(a,b){return Array.prototype.indexOf.call(a,b,void 0)}:function(a,b){if(m(a))return m(b)&&1==b.length?a.indexOf(b,0):-1;
for(var c=0;c<a.length;c++)if(c in a&&a[c]===b)return c;return-1},D=Array.prototype.forEach?function(a,b,c){Array.prototype.forEach.call(a,b,c)}:function(a,b,c){for(var d=a.length,e=m(a)?a.split(""):a,f=0;f<d;f++)f in e&&b.call(c,e[f],f,a)};
function ea(a,b){a:{var c=a.length;for(var d=m(a)?a.split(""):a,e=0;e<c;e++)if(e in d&&b.call(void 0,d[e],e,a)){c=e;break a}c=-1}return 0>c?null:m(a)?a.charAt(c):a[c]}
function fa(a){return Array.prototype.concat.apply([],arguments)}
function ha(a){var b=a.length;if(0<b){for(var c=Array(b),d=0;d<b;d++)c[d]=a[d];return c}return[]}
;function ia(a,b){this.f=a;this.g=b;this.b=0;this.a=null}
ia.prototype.get=function(){if(0<this.b){this.b--;var a=this.a;this.a=a.next;a.next=null}else a=this.f();return a};var ja=/&/g,ka=/</g,la=/>/g,ma=/"/g,na=/'/g,oa=/\x00/g,pa=/[\x00&<>"']/;var E;a:{var qa=l.navigator;if(qa){var ra=qa.userAgent;if(ra){E=ra;break a}}E=""};function sa(a){var b=F,c;for(c in b)if(a.call(void 0,b[c],c,b))return c}
;function ta(a){l.setTimeout(function(){throw a;},0)}
var G;
function ua(){var a=l.MessageChannel;"undefined"===typeof a&&"undefined"!==typeof window&&window.postMessage&&window.addEventListener&&-1==E.indexOf("Presto")&&(a=function(){var e=document.createElement("IFRAME");e.style.display="none";e.src="";document.documentElement.appendChild(e);var f=e.contentWindow;e=f.document;e.open();e.write("");e.close();var g="callImmediate"+Math.random(),h="file:"==f.location.protocol?"*":f.location.protocol+"//"+f.location.host;e=x(function(p){if(("*"==h||p.origin==h)&&
p.data==g)this.port1.onmessage()},this);
f.addEventListener("message",e,!1);this.port1={};this.port2={postMessage:function(){f.postMessage(g,h)}}});
if("undefined"!==typeof a&&-1==E.indexOf("Trident")&&-1==E.indexOf("MSIE")){var b=new a,c={},d=c;b.port1.onmessage=function(){if(void 0!==c.next){c=c.next;var e=c.F;c.F=null;e()}};
return function(e){d.next={F:e};d=d.next;b.port2.postMessage(0)}}return"undefined"!==typeof document&&"onreadystatechange"in document.createElement("SCRIPT")?function(e){var f=document.createElement("SCRIPT");
f.onreadystatechange=function(){f.onreadystatechange=null;f.parentNode.removeChild(f);f=null;e();e=null};
document.documentElement.appendChild(f)}:function(e){l.setTimeout(e,0)}}
;function H(){this.b=this.a=null}
var va=new ia(function(){return new I},function(a){a.reset()});
H.prototype.add=function(a,b){var c=va.get();c.set(a,b);this.b?this.b.next=c:this.a=c;this.b=c};
H.prototype.remove=function(){var a=null;this.a&&(a=this.a,this.a=this.a.next,this.a||(this.b=null),a.next=null);return a};
function I(){this.next=this.b=this.a=null}
I.prototype.set=function(a,b){this.a=a;this.b=b;this.next=null};
I.prototype.reset=function(){this.next=this.b=this.a=null};function wa(a){J||xa();K||(J(),K=!0);ya.add(a,void 0)}
var J;function xa(){if(l.Promise&&l.Promise.resolve){var a=l.Promise.resolve(void 0);J=function(){a.then(za)}}else J=function(){var b=za,c;
!(c="function"!=q(l.setImmediate))&&(c=l.Window&&l.Window.prototype)&&(c=-1==E.indexOf("Edge")&&l.Window.prototype.setImmediate==l.setImmediate);c?(G||(G=ua()),G(b)):l.setImmediate(b)}}
var K=!1,ya=new H;function za(){for(var a;a=ya.remove();){try{a.a.call(a.b)}catch(c){ta(c)}var b=va;b.g(a);100>b.b&&(b.b++,a.next=b.a,b.a=a)}K=!1}
;function L(){this.f=this.f;this.g=this.g}
L.prototype.f=!1;L.prototype.dispose=function(){this.f||(this.f=!0,this.A())};
L.prototype.A=function(){if(this.g)for(;this.g.length;)this.g.shift()()};function Aa(a,b){var c,d;var e=document;e=b||e;if(e.querySelectorAll&&e.querySelector&&a)return e.querySelectorAll(a?"."+a:"");if(a&&e.getElementsByClassName){var f=e.getElementsByClassName(a);return f}f=e.getElementsByTagName("*");if(a){var g={};for(c=d=0;e=f[c];c++){var h=e.className,p;if(p="function"==typeof h.split)p=0<=C(h.split(/\s+/),a);p&&(g[d++]=e)}g.length=d;return g}return f}
function Ba(a,b){for(var c=0;a;){if(b(a))return a;a=a.parentNode;c++}return null}
;var Ca=l.JSON.stringify;function M(a){L.call(this);this.m=1;this.h=[];this.i=0;this.a=[];this.b={};this.o=!!a}
z(M,L);k=M.prototype;k.subscribe=function(a,b,c){var d=this.b[a];d||(d=this.b[a]=[]);var e=this.m;this.a[e]=a;this.a[e+1]=b;this.a[e+2]=c;this.m=e+3;d.push(e);return e};
function Da(a,b,c){var d=N;if(a=d.b[a]){var e=d.a;(a=ea(a,function(f){return e[f+1]==b&&e[f+2]==c}))&&d.D(a)}}
k.D=function(a){var b=this.a[a];if(b){var c=this.b[b];if(0!=this.i)this.h.push(a),this.a[a+1]=aa;else{if(c){var d=C(c,a);0<=d&&Array.prototype.splice.call(c,d,1)}delete this.a[a];delete this.a[a+1];delete this.a[a+2]}}return!!b};
k.H=function(a,b){var c=this.b[a];if(c){for(var d=Array(arguments.length-1),e=1,f=arguments.length;e<f;e++)d[e-1]=arguments[e];if(this.o)for(e=0;e<c.length;e++){var g=c[e];Ea(this.a[g+1],this.a[g+2],d)}else{this.i++;try{for(e=0,f=c.length;e<f;e++)g=c[e],this.a[g+1].apply(this.a[g+2],d)}finally{if(this.i--,0<this.h.length&&0==this.i)for(;c=this.h.pop();)this.D(c)}}return 0!=e}return!1};
function Ea(a,b,c){wa(function(){a.apply(b,c)})}
k.clear=function(a){if(a){var b=this.b[a];b&&(D(b,this.D,this),delete this.b[a])}else this.a.length=0,this.b={}};
k.A=function(){M.J.A.call(this);this.clear();this.h.length=0};var Fa=/^(?:([^:/?#.]+):)?(?:\/\/(?:([^/?#]*)@)?([^/#?]*?)(?::([0-9]+))?(?=[/#?]|$))?([^?#]+)?(?:\?([^#]*))?(?:#([\s\S]*))?$/;function Ga(a){var b=a.match(Fa);a=b[1];var c=b[2],d=b[3];b=b[4];var e="";a&&(e+=a+":");d&&(e+="//",c&&(e+=c+"@"),e+=d,b&&(e+=":"+b));return e}
function Ha(a,b,c){if("array"==q(b))for(var d=0;d<b.length;d++)Ha(a,String(b[d]),c);else null!=b&&c.push(a+(""===b?"":"="+encodeURIComponent(String(b))))}
function Ia(a){var b=[],c;for(c in a)Ha(c,a[c],b);return b.join("&")}
var Ja=/#|$/;var O=window.yt&&window.yt.config_||window.ytcfg&&window.ytcfg.data_||{};y("yt.config_",O);function Ka(a){var b=arguments;if(1<b.length)O[b[0]]=b[1];else{b=b[0];for(var c in b)O[c]=b[c]}}
;function La(a){return a&&window.yterr?function(){try{return a.apply(this,arguments)}catch(b){Ma(b)}}:a}
function Ma(a,b){var c=n("yt.logging.errors.log");c?c(a,b,void 0,void 0,void 0):(c=[],c="ERRORS"in O?O.ERRORS:c,c.push([a,b,void 0,void 0,void 0]),Ka("ERRORS",c))}
;var Na=0;y("ytDomDomGetNextId",n("ytDomDomGetNextId")||function(){return++Na});var Oa={stopImmediatePropagation:1,stopPropagation:1,preventMouseEvent:1,preventManipulation:1,preventDefault:1,layerX:1,layerY:1,screenX:1,screenY:1,scale:1,rotation:1,webkitMovementX:1,webkitMovementY:1};
function P(a){this.type="";this.state=this.source=this.data=this.currentTarget=this.relatedTarget=this.target=null;this.charCode=this.keyCode=0;this.metaKey=this.shiftKey=this.ctrlKey=this.altKey=!1;this.clientY=this.clientX=0;this.changedTouches=this.touches=null;if(a=a||window.event){this.a=a;for(var b in a)b in Oa||(this[b]=a[b]);(b=a.target||a.srcElement)&&3==b.nodeType&&(b=b.parentNode);this.target=b;if(b=a.relatedTarget)try{b=b.nodeName?b:null}catch(c){b=null}else"mouseover"==this.type?b=a.fromElement:
"mouseout"==this.type&&(b=a.toElement);this.relatedTarget=b;this.clientX=void 0!=a.clientX?a.clientX:a.pageX;this.clientY=void 0!=a.clientY?a.clientY:a.pageY;this.keyCode=a.keyCode?a.keyCode:a.which;this.charCode=a.charCode||("keypress"==this.type?this.keyCode:0);this.altKey=a.altKey;this.ctrlKey=a.ctrlKey;this.shiftKey=a.shiftKey;this.metaKey=a.metaKey}}
P.prototype.preventDefault=function(){this.a&&(this.a.returnValue=!1,this.a.preventDefault&&this.a.preventDefault())};
P.prototype.stopPropagation=function(){this.a&&(this.a.cancelBubble=!0,this.a.stopPropagation&&this.a.stopPropagation())};
P.prototype.stopImmediatePropagation=function(){this.a&&(this.a.cancelBubble=!0,this.a.stopImmediatePropagation&&this.a.stopImmediatePropagation())};var F=n("ytEventsEventsListeners")||{};y("ytEventsEventsListeners",F);var Pa=n("ytEventsEventsCounter")||{count:0};y("ytEventsEventsCounter",Pa);
function Qa(a,b,c,d){d=void 0===d?{}:d;a.addEventListener&&("mouseenter"!=b||"onmouseenter"in document?"mouseleave"!=b||"onmouseenter"in document?"mousewheel"==b&&"MozBoxSizing"in document.documentElement.style&&(b="MozMousePixelScroll"):b="mouseout":b="mouseover");return sa(function(e){var f="boolean"==typeof e[4]&&e[4]==!!d,g;if(g=t(e[4])&&t(d))a:{g=e[4];for(var h in g)if(!(h in d)||g[h]!==d[h]){g=!1;break a}for(h in d)if(!(h in g)){g=!1;break a}g=!0}return!!e.length&&e[0]==a&&e[1]==b&&e[2]==c&&
(f||g)})}
function Ra(a){a&&("string"==typeof a&&(a=[a]),D(a,function(b){if(b in F){var c=F[b],d=c[0],e=c[1],f=c[3];c=c[4];d.removeEventListener?Sa()||"boolean"==typeof c?d.removeEventListener(e,f,c):d.removeEventListener(e,f,!!c.capture):d.detachEvent&&d.detachEvent("on"+e,f);delete F[b]}}))}
var Sa=function(a){var b=!1,c;return function(){b||(c=a(),b=!0);return c}}(function(){var a=!1;
try{var b=Object.defineProperty({},"capture",{get:function(){a=!0}});
window.addEventListener("test",null,b)}catch(c){}return a});
function Ta(a,b,c){var d=void 0===d?{}:d;if(a&&(a.addEventListener||a.attachEvent)){var e=Qa(a,b,c,d);if(!e){e=++Pa.count+"";var f=!("mouseenter"!=b&&"mouseleave"!=b||!a.addEventListener||"onmouseenter"in document);var g=f?function(h){h=new P(h);if(!Ba(h.relatedTarget,function(p){return p==a}))return h.currentTarget=a,h.type=b,c.call(a,h)}:function(h){h=new P(h);
h.currentTarget=a;return c.call(a,h)};
g=La(g);a.addEventListener?("mouseenter"==b&&f?b="mouseover":"mouseleave"==b&&f?b="mouseout":"mousewheel"==b&&"MozBoxSizing"in document.documentElement.style&&(b="MozMousePixelScroll"),Sa()||"boolean"==typeof d?a.addEventListener(b,g,d):a.addEventListener(b,g,!!d.capture)):a.attachEvent("on"+b,g);F[e]=[a,b,c,g,d]}}}
;function Ua(a){"function"==q(a)&&(a=La(a));return window.setInterval(a,250)}
;var Va={};function Wa(a){return Va[a]||(Va[a]=String(a).replace(/\-([a-z])/g,function(b,c){return c.toUpperCase()}))}
;var Q={},R=[],N=new M,Xa={};function Ya(){D(R,function(a){a()})}
function Za(a,b){b||(b=document);var c=ha(b.getElementsByTagName("yt:"+a)),d="yt-"+a,e=b||document;d=ha(e.querySelectorAll&&e.querySelector?e.querySelectorAll("."+d):Aa(d,b));return fa(c,d)}
function S(a,b){var c;"yt:"==a.tagName.toLowerCase().substr(0,3)?c=a.getAttribute(b):c=a?a.dataset?a.dataset[Wa(b)]:a.getAttribute("data-"+b):null;return c}
function $a(a,b){N.H.apply(N,arguments)}
;function ab(a){this.b=a||{};this.f=this.a=!1;a=document.getElementById("www-widgetapi-script");if(this.a=!!("https:"==document.location.protocol||a&&0==a.src.indexOf("https:"))){a=[this.b,window.YTConfig||{}];for(var b=0;b<a.length;b++)a[b].host&&(a[b].host=a[b].host.replace("http://","https://"))}}
var T=null;function U(a,b){for(var c=[a.b,window.YTConfig||{}],d=0;d<c.length;d++){var e=c[d][b];if(void 0!=e)return e}return null}
function bb(a,b,c){T||(T={},Ta(window,"message",x(a.g,a)));T[c]=b}
ab.prototype.g=function(a){if(a.origin==U(this,"host")||a.origin==U(this,"host").replace(/^http:/,"https:")){try{var b=JSON.parse(a.data)}catch(c){return}this.f=!0;this.a||0!=a.origin.indexOf("https:")||(this.a=!0);if(a=T[b.id])a.B=!0,a.B&&(D(a.u,a.C,a),a.u.length=0),a.I(b)}};function V(a,b,c){this.i=this.a=this.b=null;this.h=this[v]||(this[v]=++w);this.f=0;this.B=!1;this.u=[];this.g=null;this.m=c;this.o={};c=document;if(a=m(a)?c.getElementById(a):a)if(c="iframe"==a.tagName.toLowerCase(),b.host||(b.host=c?Ga(a.src):"https://www.youtube.com"),this.b=new ab(b),c||(b=cb(this,a),this.i=a,(c=a.parentNode)&&c.replaceChild(b,a),a=b),this.a=a,this.a.id||(a=b=this.a,a=a[v]||(a[v]=++w),b.id="widget"+a),Q[this.a.id]=this,window.postMessage){this.g=new M;db(this);b=U(this.b,"events");
for(var d in b)b.hasOwnProperty(d)&&this.addEventListener(d,b[d]);for(var e in Xa)eb(this,e)}}
k=V.prototype;k.setSize=function(a,b){this.a.width=a;this.a.height=b;return this};
k.K=function(){return this.a};
k.I=function(a){this.s(a.event,a)};
k.addEventListener=function(a,b){var c=b;"string"==typeof b&&(c=function(){window[b].apply(window,arguments)});
if(!c)return this;this.g.subscribe(a,c);fb(this,a);return this};
function eb(a,b){var c=b.split(".");if(2==c.length){var d=c[1];a.m==c[0]&&fb(a,d)}}
k.destroy=function(){this.a.id&&(Q[this.a.id]=null);var a=this.g;a&&"function"==typeof a.dispose&&a.dispose();if(this.i){a=this.a;var b=a.parentNode;b&&b.replaceChild(this.i,a)}else(a=this.a)&&a.parentNode&&a.parentNode.removeChild(a);T&&(T[this.h]=null);this.b=null;a=this.a;for(var c in F)F[c][0]==a&&Ra(c);this.i=this.a=null};
k.v=function(){return{}};
function W(a,b,c){c=c||[];c=Array.prototype.slice.call(c);b={event:"command",func:b,args:c};a.B?a.C(b):a.u.push(b)}
k.s=function(a,b){if(!this.g.f){var c={target:this,data:b};this.g.H(a,c);$a(this.m+"."+a,c)}};
function cb(a,b){for(var c=document.createElement("iframe"),d=b.attributes,e=0,f=d.length;e<f;e++){var g=d[e].value;null!=g&&""!=g&&"null"!=g&&c.setAttribute(d[e].name,g)}c.setAttribute("frameBorder",0);c.setAttribute("allowfullscreen",1);c.setAttribute("allow","accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture");c.setAttribute("title","YouTube "+U(a.b,"title"));(d=U(a.b,"width"))&&c.setAttribute("width",d);(d=U(a.b,"height"))&&c.setAttribute("height",d);var h=a.v();h.enablejsapi=
window.postMessage?1:0;window.location.host&&(h.origin=window.location.protocol+"//"+window.location.host);h.widgetid=a.h;window.location.href&&D(["debugjs","debugcss"],function(p){var u=window.location.href;var X=u.search(Ja);b:{var r=0;for(var A=p.length;0<=(r=u.indexOf(p,r))&&r<X;){var B=u.charCodeAt(r-1);if(38==B||63==B)if(B=u.charCodeAt(r+A),!B||61==B||38==B||35==B)break b;r+=A+1}r=-1}if(0>r)u=null;else{A=u.indexOf("&",r);if(0>A||A>X)A=X;r+=p.length+1;u=decodeURIComponent(u.substr(r,A-r).replace(/\+/g,
" "))}null===u||(h[p]=u)});
c.src=U(a.b,"host")+a.w()+"?"+Ia(h);return c}
k.G=function(){this.a&&this.a.contentWindow?this.C({event:"listening"}):window.clearInterval(this.f)};
function db(a){bb(a.b,a,a.h);a.f=Ua(x(a.G,a));Ta(a.a,"load",x(function(){window.clearInterval(this.f);this.f=Ua(x(this.G,this))},a))}
function fb(a,b){a.o[b]||(a.o[b]=!0,W(a,"addEventListener",[b]))}
k.C=function(a){a.id=this.h;a.channel="widget";a=Ca(a);var b=this.b;var c=Ga(this.a.src);b=0==c.indexOf("https:")?[c]:b.a?[c.replace("http:","https:")]:b.f?[c]:[c,c.replace("http:","https:")];if(!this.a.contentWindow)throw Error("The YouTube player is not attached to the DOM.");for(c=0;c<b.length;c++)try{this.a.contentWindow.postMessage(a,b[c])}catch(d){if(d.name&&"SyntaxError"==d.name)Ma(d,"WARNING");else throw d;}};function gb(a){return(0==a.search("cue")||0==a.search("load"))&&"loadModule"!=a}
function hb(a){return 0==a.search("get")||0==a.search("is")}
;function Y(a,b){if(!a)throw Error("YouTube player element ID required.");var c={title:"video player",videoId:"",width:640,height:360};if(b)for(var d in b)c[d]=b[d];V.call(this,a,c,"player");this.j={};this.l={}}
z(Y,V);function ib(a){if("iframe"!=a.tagName.toLowerCase()){var b=S(a,"videoid");b&&(b={videoId:b,width:S(a,"width"),height:S(a,"height")},new Y(a,b))}}
k=Y.prototype;k.w=function(){return"/embed/"+U(this.b,"videoId")};
k.v=function(){var a=U(this.b,"playerVars");if(a){var b={},c;for(c in a)b[c]=a[c];a=b}else a={};window!=window.top&&document.referrer&&(a.widget_referrer=document.referrer.substring(0,256));if(c=U(this.b,"embedConfig")){if(t(c))try{c=Ca(c)}catch(d){console.error("Invalid embed config JSON",d)}a.embed_config=c}return a};
k.I=function(a){var b=a.event;a=a.info;switch(b){case "apiInfoDelivery":if(t(a))for(var c in a)this.j[c]=a[c];break;case "infoDelivery":jb(this,a);break;case "initialDelivery":window.clearInterval(this.f);this.l={};this.j={};kb(this,a.apiInterface);jb(this,a);break;default:this.s(b,a)}};
function jb(a,b){if(t(b))for(var c in b)a.l[c]=b[c]}
function kb(a,b){D(b,function(c){this[c]||("getCurrentTime"==c?this[c]=function(){var d=this.l.currentTime;if(1==this.l.playerState){var e=(da()/1E3-this.l.currentTimeLastUpdated_)*this.l.playbackRate;0<e&&(d+=Math.min(e,1))}return d}:gb(c)?this[c]=function(){this.l={};
this.j={};W(this,c,arguments);return this}:hb(c)?this[c]=function(){var d=0;
0==c.search("get")?d=3:0==c.search("is")&&(d=2);return this.l[c.charAt(d).toLowerCase()+c.substr(d+1)]}:this[c]=function(){W(this,c,arguments);
return this})},a)}
k.getVideoEmbedCode=function(){var a=parseInt(U(this.b,"width"),10);var b=parseInt(U(this.b,"height"),10),c=U(this.b,"host")+this.w();pa.test(c)&&(-1!=c.indexOf("&")&&(c=c.replace(ja,"&amp;")),-1!=c.indexOf("<")&&(c=c.replace(ka,"&lt;")),-1!=c.indexOf(">")&&(c=c.replace(la,"&gt;")),-1!=c.indexOf('"')&&(c=c.replace(ma,"&quot;")),-1!=c.indexOf("'")&&(c=c.replace(na,"&#39;")),-1!=c.indexOf("\x00")&&(c=c.replace(oa,"&#0;")));a='<iframe width="'+a+'" height="'+b+'" src="'+c+'" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>';
return a};
k.getOptions=function(a){return this.j.namespaces?a?this.j[a].options||[]:this.j.namespaces||[]:[]};
k.getOption=function(a,b){if(this.j.namespaces&&a&&b)return this.j[a][b]};function Z(a,b){var c={title:"Thumbnail",videoId:"",width:120,height:68};if(b)for(var d in b)c[d]=b[d];V.call(this,a,c,"thumbnail")}
z(Z,V);function lb(a){if("iframe"!=a.tagName.toLowerCase()){var b=S(a,"videoid");if(b){b={videoId:b,events:{}};b.width=S(a,"width");b.height=S(a,"height");b.thumbWidth=S(a,"thumb-width");b.thumbHeight=S(a,"thumb-height");b.thumbAlign=S(a,"thumb-align");var c=S(a,"onclick");c&&(b.events.onClick=c);new Z(a,b)}}}
Z.prototype.w=function(){return"/embed/"+U(this.b,"videoId")};
Z.prototype.v=function(){return{player:0,thumb_width:U(this.b,"thumbWidth"),thumb_height:U(this.b,"thumbHeight"),thumb_align:U(this.b,"thumbAlign")}};
Z.prototype.s=function(a,b){Z.J.s.call(this,a,b?b.info:void 0)};y("YT.PlayerState.UNSTARTED",-1);y("YT.PlayerState.ENDED",0);y("YT.PlayerState.PLAYING",1);y("YT.PlayerState.PAUSED",2);y("YT.PlayerState.BUFFERING",3);y("YT.PlayerState.CUED",5);y("YT.get",function(a){return Q[a]});
y("YT.scan",Ya);y("YT.subscribe",function(a,b,c){N.subscribe(a,b,c);Xa[a]=!0;for(var d in Q)eb(Q[d],a)});
y("YT.unsubscribe",function(a,b,c){Da(a,b,c)});
y("YT.Player",Y);y("YT.Thumbnail",Z);V.prototype.destroy=V.prototype.destroy;V.prototype.setSize=V.prototype.setSize;V.prototype.getIframe=V.prototype.K;V.prototype.addEventListener=V.prototype.addEventListener;Y.prototype.getVideoEmbedCode=Y.prototype.getVideoEmbedCode;Y.prototype.getOptions=Y.prototype.getOptions;Y.prototype.getOption=Y.prototype.getOption;R.push(function(a){a=Za("player",a);D(a,ib)});
R.push(function(){var a=Za("thumbnail");D(a,lb)});
"undefined"!=typeof YTConfig&&YTConfig.parsetags&&"onload"!=YTConfig.parsetags||Ya();var mb=n("onYTReady");mb&&mb();var nb=n("onYouTubeIframeAPIReady");nb&&nb();var ob=n("onYouTubePlayerAPIReady");ob&&ob();}).call(this);
