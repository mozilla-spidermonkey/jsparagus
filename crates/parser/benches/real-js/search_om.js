/* release date : 2014-09-18 */
function document_write(a){document.write(a)}function NXTT_Display(a){var b=NXTT.div,c,d,e;a>1&&b.style.display=="none"&&(b.style.left=b.style.right="",b.style.width="",b.style.visibility="hidden",b.style.display="block",NXTT.width=b.scrollWidth,NXTT.env.maxWidth&&NXTT.env.maxWidth<NXTT.width&&(b.style.width=NXTT.env.maxWidth+"px",b.firstChild&&(NXTT.width=Math.max(b.scrollWidth,b.firstChild.scrollWidth))),b.style.display="none",b.style.visibility="visible");if(!NXTT.show)return;e=document.body.scrollWidth-NXTT.env.padding[1]-NXTT.width,d=NXTT.px+NXTT.env.offset[1],d>e&&(NXTT.px-NXTT.env.offset[3]>=NXTT.env.padding[3]+NXTT.width?d-=NXTT.width+NXTT.env.offset[1]+NXTT.env.offset[3]:d=e),c=NXTT.py+NXTT.env.offset[2],b.style.top=c+"px",b.style.left=d+"px",a&&(b.style.display="")}function NXTT_SetHTML(a){NXTT.div.innerHTML=""+(NXTT.env.head?NXTT.env.head:"")+a+(NXTT.env.tail?NXTT.env.tail:""),NXTT.ready=!0,NXTT_Display(2)}function NXTT_onload(a){a||(a=window.event),NXTT.env.type=="url"&&(a.currentTarget?a.currentTarget:a.srcElement).id==NXTT.env.frameId&&NXTT_SetHTML(document.getElementById(NXTT.env.frameId).contentWindow.document.body.innerHTML)}function NXTT_Load(){switch(NXTT.env.type){case"id.value":NXTT_SetHTML(document.getElementById(NXTT.id).value);break;case"id.html":NXTT_SetHTML(document.getElementById(NXTT.id).innerHTML);break;case"html":NXTT_SetHTML(NXTT.id);break;case"url":NXTT.ready=!1;var a=document.getElementById(NXTT.env.frameId);a.attachEvent?(a.detachEvent("onload",NXTT_onload),a.attachEvent("onload",NXTT_onload)):a.onload=NXTT_onload,a.src=NXTT.id}}function NXTT_HideDisplay(){NXTT.show||(NXTT.div.style.display="none")}function NXTT_HideRequest(){NXTT.show=!1,NXTT.env.delay?setTimeout("NXTT_HideDisplay()",NXTT.env.delay*1e3):NXTT_HideDisplay()}function NXTT_HideNow(){NXTT.show=!1,NXTT_HideDisplay()}function NXTT_Move(a){a||(a=window.event),NXTT.px=a.pageX?a.pageX:document.body.scrollLeft+a.clientX-document.body.clientLeft,NXTT.py=a.pageY?a.pageY:document.documentElement.scrollTop+a.clientY-document.body.clientTop-1,NXTT.ready&&NXTT_Display(0)}function NXTT_Show(a,b,c,d){a||(a=window.event);var e=navigator.userAgent;if(e.indexOf("MSIE")>=0){var f=navigator.appVersion.split("; ")[1].split(" ")[1];if(f<5)return}if(!NXTT.setDisable){var g=document.getElementsByTagName("link");g[0].disabled&&(NXTT.disable=!0),NXTT.setDisable=!0}if(NXTT.setDisable&&NXTT.disable)return;NXTT.env.followMouse&&(b.onmousemove=NXTT_Move),b.onmouseout=NXTT_HideRequest,NXTT.px=a.pageX?a.pageX:document.body.scrollLeft+a.clientX-document.body.clientLeft,NXTT.py=a.pageY?a.pageY:document.documentElement.scrollTop+a.clientY-document.body.clientTop-1;if(c==NXTT.env&&d==NXTT.id){NXTT.show=!0,NXTT.ready&&NXTT_Display(1);return}NXTT.div?NXTT_HideNow():(NXTT.div=document.createElement("div"),NXTT.div.style.display="none",NXTT.div.style.position="absolute",NXTT.div.style.borderWidth=0,c.zIndex&&(NXTT.div.style.zIndex=c.zIndex),document.body.firstChild.insertAdjacentElement?document.body.firstChild.insertAdjacentElement("BeforeBegin",NXTT.div):document.body.appendChild(NXTT.div)),NXTT.env=c,NXTT.id=d,NXTT.env.mouseontooltip?NXTT.div.onmouseover=NXTT.div.onmousemove=function(){NXTT.show=!0}:NXTT.div.onmouseover=NXTT.div.onmousemove=function(a){NXTT.show=!0,NXTT_Move(a)},NXTT.div.onmouseout=NXTT_HideRequest,NXTT.show=!0,NXTT_Load(d)}var NXTT={div:null,ready:!1,show:!1,env:0,id:null,px:0,py:0,width:0,Show:NXTT_Show,Hide:NXTT_HideNow,disable:!1,setDisable:!1}