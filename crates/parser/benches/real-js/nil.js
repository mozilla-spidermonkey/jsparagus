"undefined"==typeof nil&&(nil={}),function(){function n(n){x&&x(n);var e;if("http:"!=T.protocol&&"https:"!=T.protocol){return !1;}e=T.protocol;var t=R?E:A,r=new Image(1,1);return r.src=e+"//"+t+n,!0;}function e(){var n=[],e=-1;for(var t in h){n[++e]=t+"="+I(h[t]);}for(t in k){n[++e]=t+"="+I(k[t]);}return n[++e]="t="+(new Date).getTime(),n.join("&");}function t(n){var e=n.length;if(e<1){return 0;}var t=n[0];switch(t){case"cv":B=(new Date).getTime(),r("cv",n[1],"","",""),g(n[2],n[3]);break;case"leave":var o=i();if(o<=0){return 0;}r("event","action","leave","",""+o),g(n[5],n[6]),B=null;break;case"event":r("event",n[1],n[2],n[3],n[4]),g(n[5],n[6]);}return 1;}function r(n,e,t,r,i){k.h=n||"",k.ec=e||"",k.ea=t||"",k.el=r||"",k.ev=i||"";}function i(){return null===B?0:(new Date).getTime()-B;}function o(n){return"object"==typeof n;}function c(n){return"string"==typeof n;}function a(){h={},k={};}function u(n,e){var t=D.referrer?D.referrer:"";h.s=n,h.c=e,h.r=t;}function f(){var n;try{n=j.userLanguage||j.language||"";}catch(e){}h.l=n,h.v=b;}function l(){var n=s("og:url","content");n||(n=T.href?T.href:""),h.u=n,d();}function v(n){var e={};return n.replace(new RegExp("([^?=&]+)(=([^&]*))?","g"),function(n,t,r,i){e[t]=i;}),e;}function d(){var n=window.location.search,e=v(n),t=["cd_source","cd_type","cd_contents","cd_value"];for(var r in t){var i=t[r];i in e&&e[i]&&p(h,i,e[i]);}}function s(n,e){for(var t=document.getElementsByTagName("meta"),r=0;r<t.length;r++){if(t[r].getAttribute("property")===n){return t[r].getAttribute(e);}}}function m(r){t(r)&&(n(e()),k={});}function g(n,e){if(o(n)){for(var t in n){c(n[t])?p(h,t,n[t]):C[t]&&typeof n[t]==C[t]&&p(h,t,n[t].toString());}}c(n)&&c(e)&&p(h,n,e);}function p(n,e,t){var r={referrer:"r",gdid:"m",uri:"u",user_type:"f",cd_source:"cds",cd_type:"cdt",cd_contents:"cdc",cd_value:"cdv"};if(e in r){n[r[e]]=t;}else{var i=e.split("_"),o=i.pop();if(!o.search(/0-9+/)){return;}var c=i.join("_"),a={dimension:{limit:10,key:"d"},metric:{limit:10,key:"m"},optional:{limit:10,key:"o"}};if(!(c in a)){return;}var u=parseInt(o);if(u>a[c].limit){return;}n[a[c].key+o]=t;}}function w(n,e){a(),u(n,e),f(),l();}function y(n){x=n;}function _(n){R=n;}var h={},k={},b="1.0.6",j=window.navigator,T=window.location,D=document,I=(D.referrer,encodeURIComponent),A=(window.java||self.java?java.awt.Toolkit.getDefaultToolkit():null,"nil.naver.com/m?"),E="dev.nil.naver.com/m?",R=!1,x=null,B=null,C={cd_value:"number"};nil.init=function(n,e){w(n,e);},nil.add=function(n,e){g(n,e);},nil.send=function(){m(arguments);},nil.callback=function(n){y(n);},nil.setDevMode=function(n){_(n);};}();