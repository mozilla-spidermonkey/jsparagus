var FlagFeedback;(function(n){function d(n){n=n||window.event;var i=n.target||n.srcElement;t&&!t.contains(i)&&t.offsetHeight>0&&a()}function g(n){var i,r;n=n||window.event;i=n.target||n.srcElement;t&&t.contains(i)&&(r=n?n.which?n.which:n.keyCode:n.keyCode,r==ut?(i.tagName=="INPUT"||i.className=="buttonLink"||i.id=="fbdialogcl")&&i.click():r==ft?(i.className=="buttonLink"||i.id=="fbdialogcl")&&(i.click(),c(n)):r==et&&(a(),c(n)))}function nt(n){h&&!t.contains(n.target)&&(c(n),t.focus())}function c(n){sj_sp(n);sj_pd(n)}function l(){sj_ue(_d,"click",d);sj_ue(_d,"keydown",g);sj_ue(_d,"focusin",nt);sj_evt.unbind("ajax.unload",l)}function ot(){s=document.activeElement;var n=this.metadata;n&&st(n.turl,n.maw,n.mah)}function st(n,u,f){r.innerHTML="";var e=_d.createElement("img");e.src=n;u&&f&&(u>250?(e.width=250,e.height=f*250/u):(e.width=u,e.height=f));r.appendChild(e);t.style.display="block";h=!0;i.focus()}function ht(){(i.checked||f.checked||e.checked||o.checked)&&(u.style.display="none")}function a(){t.style.display="none";v.style.display="block";y.style.display="none";r.style.display="block";u.style.display="none";w.style.display="block";b.style.display="block";k.style.display="none";i.checked=!1;f.checked=!1;e.checked=!1;o.checked=!1;var n=this.metadata;n&&tt("flagClose",null,n.ns,n.k);h=!1;s&&s.focus();sj_evt.fire(rt)}function ct(){var t,r,n,s;if(!i.checked&&!f.checked&&!e.checked&&!o.checked){u.style.display="block";u.focus();return}if(t=[],i.checked&&t.push("irrelevant"),f.checked&&t.push("offensive"),e.checked&&t.push("adult"),o.checked&&t.push("childabuse"),r=t.join(","),n=this.metadata,n&&(tt("flagSubmit",r,n.ns,n.k),s=p.getAttribute("fbposturl"),s)){var h=window.location.href.match("(images|videos)"),l=h?h[0]:"",c=window.location.href.match(/q=(.+?)(&|$)/),a=c?c[1]:"",v=p.getAttribute("ss"),y=!!n.md5&&n.md5.length>0?n.md5:null;ReportResult.send(s,l,a,"External",encodeURIComponent(n.imgurl),encodeURIComponent(n.surl),encodeURIComponent(n.turl),y,null,r,n.src,v)}lt()}function tt(n,t,i,r){if(typeof mmLog!="undefined"&&mmLog){var u=['{"T":"CI.Click","Name":"',n,'","Meta":"',t,'","AppNS":"',i,'","K":"',r,".1",'","TS":',sb_gt(),"}"];mmLog(u.join(""))}}function lt(){v.style.display="none";y.style.display="block";r.style.display="none";w.style.display="none";b.style.display="none";k.style.display="block";it.focus()}var t=_ge("fbdialog"),v=_ge("fbdialog_message"),y=_ge("fbthankyou_message"),p=_ge("fbdialog_container"),r=_ge("fbdialog_thumb_container"),u=_ge("fbdialog_errormessage"),w=_ge("checkbox_region"),i=_ge("irrelevant_mark_checkbox"),f=_ge("offensive_mark_checkbox"),e=_ge("adult_mark_checkbox"),o=_ge("childabuse_mark_checkbox"),b=_ge("fbdialog_buttons"),k=_ge("fbthankyou_button"),it=_ge("adult_button_close"),rt="flagfeedback_close",ut=13,ft=32,et=27,s,h=!1;sj_be(_d,"click",d);sj_be(_d,"keydown",g);sj_be(_d,"focusin",nt);sj_be(_d,"unload",l);sj_evt.bind("ajax.unload",l);n.c=ot;n.p=ht;n.s=ct;n.h=a})(FlagFeedback||(FlagFeedback={}))