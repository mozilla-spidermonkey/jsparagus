window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.ContextAd=eg.Class({_$element:null,_oTemplate:null,_oContextAd:null,_oTimer:null,_AD_EXTENSION_TYPE:{1:"tel",2:"map",3:"reserve",4:"talk",5:"calculate",6:"addTitle",7:"addContent",8:"subLink",
9:"price"},construct:function(htOption){this._oContextAd=htOption.contextAdList||null;if(this._oContextAd==null||this._oContextAd.length<1)return;this._setElement();this._showAd()},
_setElement:function(){this._$element={};this._$element["detailContextAd"]=$("#detailContextAd");this._$element["contextAdItemArea"]=$("#contextAdItemArea");this._oTemplate={};this._oTemplate["detailContextAdTpl"]=
doT.template($("#detailContextAdTpl").html())},
_showAd:function(){if(this._$element["contextAdItemArea"].existElements()&&this._oTemplate["detailContextAdTpl"]){this._$element["contextAdItemArea"].html(this._oTemplate["detailContextAdTpl"]({adList:this._getRevisedAdContextList()}));
this._setEvent();this._$element["detailContextAd"].show()}},
_setEvent:function(){this._$element["iconArea"]=this._$element["contextAdItemArea"].find("._iconArea");if(this._$element["iconArea"].existElements()){this._$element["iconArea"].on("mouseenter",this._onFocusedIconArea.bind(this));
this._$element["iconArea"].on("mouseleave",this._onBlurredIconArea.bind(this));this._$element["iconArea"].on("focusin",this._onFocusedIconArea.bind(this));this._$element["iconArea"].on("focusout",this._onBlurredIconArea.bind(this))}},
_onFocusedIconArea:function(oEvent){if(!oEvent)return;var $el=$(oEvent.target);var $elParent=$el.closest("._iconArea");if($elParent.data("timer")){clearTimeout($elParent.data("timer"));$elParent.data("timer",
null)}var $elTooltipArea=$.getSingle("._tooltip",$elParent);$elTooltipArea.show()},
_onBlurredIconArea:function(oEvent){if(!oEvent)return;var $el=$(oEvent.target);var $elParent=$el.closest("._iconArea");if(!$elParent.data("timer"))$elParent.data("timer",setTimeout(function(){var $elTooltipArea=
$.getSingle("._tooltip",$elParent);$elTooltipArea.hide()},100))},
_getRevisedAdContextList:function(){var aAdContextList=this._oContextAd.ads;var sKeyword=this._oContextAd.keyword||"";var aRevisedAdContextList=[];for(var i=0,len=aAdContextList.length;i<len;i++){var oAdContext=
aAdContextList[i];var aAdExtensions=oAdContext.adExtensions;var bHasExtensions=aAdExtensions&&aAdExtensions.length>0;aRevisedAdContextList.push({"title":sKeyword?(oAdContext.title||"").split(sKeyword).join('\x3cstrong class\x3d"hl"\x3e'+
sKeyword+"\x3c/strong\x3e"):oAdContext.title||"","description":sKeyword?(oAdContext.desc||"").split(sKeyword).join('\x3cstrong class\x3d"hl"\x3e'+sKeyword+"\x3c/strong\x3e"):oAdContext.desc||"","clickUrl":oAdContext.cUrl||
"","viewUrl":oAdContext.vUrl||"","telUrl":oAdContext.tUrl||"","telLink":oAdContext.telLink||"","phone":oAdContext.phone||"","isMobile":oAdContext.isMobile===true,"isNaverPay":oAdContext.isNaverPay===true,
"isNaverLogin":oAdContext.isNaverLogin===true,"isNaverBooking":oAdContext.isNaverBooking===true,"extensions":bHasExtensions?this._getExtensionsInfo(aAdExtensions):{}})}return aRevisedAdContextList},
_getExtensionsInfo:function(aAdExtensions){if(!aAdExtensions)return{};var oExtensionsInfo={};for(var i=0,len=aAdExtensions.length;i<len;i++){var oCurrentExtension=aAdExtensions[i];var sExtensionType=this._AD_EXTENSION_TYPE[oCurrentExtension.type];
if(sExtensionType)oExtensionsInfo[sExtensionType]=oCurrentExtension}return oExtensionsInfo}});