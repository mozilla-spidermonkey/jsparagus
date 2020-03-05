window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.VoteQuestionDetail=eg.Class({_$element:null,_$template:null,_oVoteAnswerRegisterAjax:null,_oVoteAnswerModifyAjax:null,_oVoteAnswerDeleteAjax:null,_nListDirId:null,_nDirId:0,_nDocId:0,
_nQuestionListPage:1,_sQuestionListState:null,_sQuestionTitle:null,_sTempField:null,_sMdu:null,_nVoteItemCnt:0,_bHasSelectedItem:false,_bIsMyQuestion:false,_sState:null,_oVoteQuestionListInfoMap:null,_nMyAnswerNo:0,
_bStopEvent:false,_URL:{VOTE_ANSWER_REGISTER_AJAX:"/ajax/answerAjax.nhn",VOTE_ANSWER_MODIFY_AJAX:"/ajax/answerModifyAjax.nhn",VOTE_ANSWER_DELETE_AJAX:"/ajax/answerDeleteAjax.nhn",VOTE_QUESTION_LIST_AJAX:"/ajax/detail/voteQuestionListAjax.nhn"},
_VOTE_QUESTION_STATE:{WAIT:"WAIT",ING:"ING",TERMINATE:"TERMINATE"},_MESSAGE:{UNKNOWN_VOTE_QUESTION:"\uc9c8\ubb38 \uc815\ubcf4\ub97c \uac00\uc838\uc624\ub294 \ub370 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4. \ud398\uc774\uc9c0 \uc0c8\ub85c\uace0\uce68 \ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
VOTE_ANSWER_REGISTER_PROCESSING:"\ub2f5\ubcc0\uc744 \ub4f1\ub85d\ud558\uace0 \uc788\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ub9cc \uae30\ub2e4\ub824\uc8fc\uc138\uc694.",VOTE_ANSWER_MODIFY_PROCESSING:"\ub2f5\ubcc0\uc744 \uc218\uc815\ud558\uace0 \uc788\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ub9cc \uae30\ub2e4\ub824\uc8fc\uc138\uc694.",
VOTE_ANSWER_DELETE_PROCESSING:"\ub2f5\ubcc0\uc744 \ucde8\uc18c\ud558\uace0 \uc788\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ub9cc \uae30\ub2e4\ub824\uc8fc\uc138\uc694.",FAILED_TO_ANSWER_REGISTER:"\uc608\uae30\uce58 \uc54a\uc740 \uc624\ub958\ub85c \ub2f5\ubcc0 \ub4f1\ub85d\uc5d0 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc \ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
FAILED_TO_ANSWER_MODIFY:"\uc608\uae30\uce58 \uc54a\uc740 \uc624\ub958\ub85c \ub2f5\ubcc0 \uc218\uc815\uc5d0 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc \ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
FAILED_TO_ANSWER_DELETE:"\uc608\uae30\uce58 \uc54a\uc740 \uc624\ub958\ub85c \ub2f5\ubcc0 \ucde8\uc18c\uc5d0 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc \ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694."},
_VOTE_ITEM_MIN_COUNT:2,_VOTE_ITEM_MIN_COUNT_FOR_LIST_TYPE:3,construct:function(htOption){if(htOption){this._nDirId=htOption.dirId||0;this._nDocId=htOption.docId||0;this._nQuestionListPage=htOption.questionListPage||
1;this._sQuestionListState=htOption.questionListState||"ING_TERMINATE";this._sQuestionTitle=htOption.questionTitle||"";this._sTempField=htOption.tempField||"";this._sMdu=htOption.mdu||"";this._nVoteItemCnt=
htOption.voteItemCnt||0;this._bHasSelectedItem=htOption.hasSelectedItem||false;this._bIsMyQuestion=htOption.isMyQuestion||false;this._sState=htOption.state||this._VOTE_QUESTION_STATE.ING;this._nMyAnswerNo=
htOption.myAnswerNo||0;this.oTicketLayer=htOption.oTicketLayer}this._setElement();this._setEvent();this._showListTypeThumbnail()},
_setElement:function(){this._$element={};this._$template={};this._$element["voteQuestionArea"]=$.getSingle("div._voteQuestionArea");this._$element["voteItemListArea"]=$.getSingle("ul._voteItemListArea",
this._$element["voteQuestionArea"]);this._$element["thumbnailList"]=$("div._thumbnail",this._$element["voteQuestionArea"]);this._$element["multiVoteItemListArea"]=$("ul._multiVoteItemListArea",this._$element["voteQuestionArea"]);
this._$element["voteItemList"]=this._$element["voteItemListArea"].find("li._voteItemList");this._$element["voteItemResultListArea"]=$.getSingle("ul._voteItemResultListArea",this._$element["voteQuestionArea"]);
this._$element["voteItemResultList"]=$("li._voteItemResultList",this._$element["voteItemResultListArea"]);this._$element["totalVoteCnt"]=$.getSingle("span._totalVoteCnt");this._$element["quizLayer"]=$.getSingle("div._quizLayer");
this._$element["reselectBtn"]=$.getSingle("a._reselect");this._$element["reselectCancelBtn"]=$.getSingle("a._reselectCancel");this._$element["cancelVoteItemDimmedLayer"]=$("#cancelVoteItemDimmedLayer");
this._$element["cancelVoteItemLayer"]=$.getSingle("div._cancelVoteItemLayer",this._$element["cancelVoteItemDimmedLayer"]);var $elQuizLayerTpl=$("#quizAnswerLayerTpl");if($elQuizLayerTpl.existElements())this._$template["quizAnswerLayer"]=
doT.template($elQuizLayerTpl.html())},
_setEvent:function(){if(this._$element["voteItemList"].existElements())this._$element["voteItemList"].on("click",this._onClickVoteItemList.bind(this));if(this._$element["quizLayer"].existElements())this._$element["quizLayer"].on("click",
this._onClickQuizLayer.bind(this));if(this._$element["reselectBtn"].existElements())this._$element["reselectBtn"].on("click",this._onClickReselectBtn.bind(this));if(this._$element["reselectCancelBtn"].existElements())this._$element["reselectCancelBtn"].on("click",
this._onClickReselectCancelBtn.bind(this));if(this._$element["cancelVoteItemLayer"].existElements())this._$element["cancelVoteItemLayer"].on("click",this._onClickCancelVoteItemLayer.bind(this))},
_showListTypeThumbnail:function(){if(!this._$element["thumbnailList"].existElements())return;if(this._$element["multiVoteItemListArea"].existElements())this._$element["multiVoteItemListArea"].removeClass("no_thum_all")},
_showReselectBtn:function(){if(this._$element["reselectBtn"].existElements())this._$element["reselectBtn"].show()},
_hideReselectBtn:function(){if(this._$element["reselectBtn"].existElements())this._$element["reselectBtn"].hide()},
_showReselectCancelBtn:function(){if(this._$element["reselectCancelBtn"].existElements())this._$element["reselectCancelBtn"].show()},
_hideReselectCancelBtn:function(){if(this._$element["reselectCancelBtn"].existElements())this._$element["reselectCancelBtn"].hide()},
_isProcessingAjax:function(){if(this._oVoteAnswerRegisterAjax)return true;if(this._oVoteAnswerModifyAjax)return true;if(this._oVoteAnswerDeleteAjax)return true;return false},
_onClickReselectBtn:function(oEvent){if(this._isProcessingAjax()){oEvent.preventDefault();oEvent.stopPropagation();return}if(this._$element["voteItemResultListArea"].existElements()){var $elVoteItemResult=
$.getSingle("li._voteItemResultList.highlight",this._$element["voteItemResultListArea"]);if($elVoteItemResult.existElements())this._hideResultList($elVoteItemResult)}if(this._$element["voteItemListArea"].existElements()){var $elVoteItem=
$.getSingle("li._voteItemList.on",this._$element["voteItemListArea"]);if($elVoteItem.existElements())$elVoteItem.addClass("is_selected")}this._addReselectClass();this._hideReselectBtn();this._showReselectCancelBtn();
this._bStopEvent=false;oEvent.preventDefault();oEvent.stopPropagation()},
_onClickReselectCancelBtn:function(oEvent){if(this._isProcessingAjax()){oEvent.preventDefault();oEvent.stopPropagation();return}if(this._$element["voteItemResultListArea"].existElements()){var $elVoteItemResult=
$.getSingle("li._voteItemResultList.highlight",this._$element["voteItemResultListArea"]);if($elVoteItemResult.existElements())this._showResultList($elVoteItemResult)}if(this._$element["voteItemListArea"].existElements()){var $elVoteItem=
$.getSingle("li._voteItemList.on",this._$element["voteItemListArea"]);if($elVoteItem.existElements())$elVoteItem.removeClass("is_selected")}this._removeReselectClass();this._showReselectBtn();this._hideReselectCancelBtn();
oEvent.preventDefault();oEvent.stopPropagation()},
_addReselectClass:function(){if(this._$element["voteQuestionArea"].existElements()){this._$element["voteQuestionArea"].addClass("_reselect");this._$element["voteQuestionArea"].addClass("re_select")}},
_removeReselectClass:function(){if(this._$element["voteQuestionArea"].existElements()){this._$element["voteQuestionArea"].removeClass("_reselect");this._$element["voteQuestionArea"].removeClass("re_select")}},
_onClickVoteItemList:function(oEvent){if(this._bStopEvent){oEvent.preventDefault();oEvent.stopPropagation();return}this._bStopEvent=true;if(naver.kin.pc.common.checkRos()){oEvent.preventDefault();oEvent.stopPropagation();
return}if(!naver.isLogin){naver.kin.pc.common.openLoginPopup();oEvent.preventDefault();oEvent.stopPropagation();return}var el=oEvent.target;if(!$(el).hasClass("_voteItemList")){var $elVoteItemList=$(el).parents("._voteItemList");
if($elVoteItemList.existElements())el=$elVoteItemList.get(0);else{oEvent.preventDefault();oEvent.stopPropagation();return}}var $elVoteItem=$(el);if(this._$element["voteQuestionArea"].existElements()&&this._$element["voteQuestionArea"].hasClass("_reselect")){var $elOriginVoteItem=
$.getSingle("li._voteItemList.on",this._$element["voteItemListArea"]);if(!$elOriginVoteItem.existElements()){oEvent.preventDefault();oEvent.stopPropagation();return}var nOriginVoteItemNo=parseInt($elOriginVoteItem.data("itemNo"),
10);var nNewVoteItemNo=parseInt($elVoteItem.data("itemNo"),10);if(nNewVoteItemNo<0||nOriginVoteItemNo<0){oEvent.preventDefault();oEvent.stopPropagation();return}if(nOriginVoteItemNo===nNewVoteItemNo){naver.kin.pc.common.nClicks("evs.itemcancel",
"","",oEvent);this._showCancelVoteItemLayer()}else{naver.kin.pc.common.nClicks("evs.otheritem","","",oEvent);this._voteAnswerModifyAjax($elVoteItem,$elOriginVoteItem)}}else if(this._bIsAnswerable()){naver.kin.pc.common.nClicks("evs.answer",
"","",oEvent);this._voteAnswerRegisterAjax($elVoteItem)}else this._bStopEvent=false;oEvent.preventDefault();oEvent.stopPropagation()},
_voteAnswerRegisterAjax:function($elVoteItem){if(!$elVoteItem.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}var nVoteItemNo=parseInt($elVoteItem.data("itemNo"),
10);var sVoteItemTitle=$elVoteItem.data("itemTitle");if(!sVoteItemTitle||isNaN(nVoteItemNo)){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}if(this._oVoteAnswerRegisterAjax){alert(this._MESSAGE.VOTE_ANSWER_REGISTER_PROCESSING);
this._bStopEvent=false;return}var $elVoteItemResult=$("#voteItemResult"+nVoteItemNo);if(!$elVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}this._selectVoteItem($elVoteItem,
$elVoteItemResult);var oParam={dirId:this._nDirId||0,docId:this._nDocId||0,selectedItemNo:nVoteItemNo,delegateFlag:"N",supportFlag:"N",openFlag:"N",rssFlag:"N",title:this._sQuestionTitle||"",contents:sVoteItemTitle,
tempField:this._sTempField||"",inputDevice:"PC",mdu:this._sMdu};var oOption={url:this._URL.VOTE_ANSWER_REGISTER_AJAX,data:oParam,method:"post",timeout:5E3,success:this._onSucceedInVoteAnswerRegisterAjax.bind(this,
$elVoteItem,$elVoteItemResult,nVoteItemNo),error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")this._onFailInVoteAnswerRegisterAjax(this,naver.kin.pc.common.MESSAGE.TIMEOUT);else this._onFailInVoteAnswerRegisterAjax(this,
naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)};
this._oVoteAnswerRegisterAjax=$.ajax(oOption)},
_onSucceedInVoteAnswerRegisterAjax:function($elVoteItem,$elVoteItemResult,nVoteItemNo,oResponse){if(!$elVoteItem||!$elVoteItemResult){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}this._oVoteAnswerRegisterAjax=
null;if(oResponse&&oResponse.isSuccess&&oResponse.result&&oResponse.result.length>0){var oResult=oResponse.result[0];var nTotalVoteCnt=oResult.totalVoteCnt||0;var oVoteCntInfoMap=oResult.voteCntInfoMap||
{};this._nMyAnswerNo=oResult.answerNo||0;this._setResultList(nTotalVoteCnt,oVoteCntInfoMap,nVoteItemNo);this._setTotalVoteCnt(nTotalVoteCnt);if(this._$element["quizLayer"].existElements()){var nQuizAnswerItemNo=
oResult.quizAnswerItemNo;var isCorrectAnswer=nVoteItemNo===nQuizAnswerItemNo;var $elAnswerItem=$("#voteItem"+nQuizAnswerItemNo);var sTitle=$elAnswerItem.data("itemTitle");sTitle=sTitle.replace(/\\u([\d\w]{4})/gi,
function(match,grp){return String.fromCharCode(parseInt(grp,16))});
this._$element["quizLayer"].html(this._$template["quizAnswerLayer"]({"answerTitle":decodeURIComponent(sTitle),"quizAnswerDescription":oResult.quizAnswerDescription||"","quizAnswerLinkUrl":oResult.quizAnswerLinkUrl||
"","isCorrectAnswer":isCorrectAnswer}));this._$element["quizLayer"].show()}if((oResult.rouletteTicketCreated||oResult.wordChainTicketCreated)&&this.oTicketLayer)this.oTicketLayer.showLayer(oResult);else try{naver.kin.pc.common.Bingo.checkAndShowBingoCompleteLayer(oResponse)}catch(e){}this._showResultList($elVoteItemResult);
if(!this._$element["quizLayer"].existElements()){this._showReselectBtn();this._hideReselectCancelBtn()}}else{this._unselectVoteItem($elVoteItem,$elVoteItemResult);alert(this._MESSAGE.FAILED_TO_ANSWER_REGISTER);
this._bStopEvent=false}},
_onFailInVoteAnswerRegisterAjax:function($elVoteItem,$elVoteItemResult,sErrorMessage){if(!this._oVoteAnswerRegisterAjax)this._oVoteAnswerRegisterAjax.abort();this._unselectVoteItem($elVoteItem,$elVoteItemResult);
if(sErrorMessage)alert(this._MESSAGE.FAILED_TO_ANSWER_REGISTER);this._bStopEvent=false},
_voteAnswerModifyAjax:function($elNewVoteItem,$elOriginVoteItem){if(!$elNewVoteItem.existElements()||!$elOriginVoteItem.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;
return}var sNewVoteItemTitle=$elNewVoteItem.data("itemTitle");var nNewVoteItemNo=parseInt($elNewVoteItem.data("itemNo"),10);var nOriginVoteItemNo=parseInt($elOriginVoteItem.data("itemNo"),10);if(!sNewVoteItemTitle||
isNaN(nNewVoteItemNo)||isNaN(nOriginVoteItemNo)){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}if(this._oVoteAnswerModifyAjax){alert(this._MESSAGE.VOTE_ANSWER_MODIFY_PROCESSING);
this._bStopEvent=false;return}var $elOriginVoteItemResult=$("#voteItemResult"+nOriginVoteItemNo);if(!$elOriginVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=
false;return}var $elNewVoteItemResult=$("#voteItemResult"+nNewVoteItemNo);if(!$elNewVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}this._unselectVoteItem($elOriginVoteItem,
$elOriginVoteItemResult);this._selectVoteItem($elNewVoteItem,$elNewVoteItemResult);var oParam={dirId:this._nDirId||0,docId:this._nDocId||0,answerNo:this._nMyAnswerNo||0,selectedItemNo:nNewVoteItemNo,canceledItemNo:nOriginVoteItemNo,
delegateFlag:"N",supportFlag:"N",openFlag:"N",rssFlag:"N",title:this._sQuestionTitle||"",contents:sNewVoteItemTitle,tempField:this._sTempField||"",inputDevice:"PC",mdu:this._sMdu};var oOption={url:this._URL.VOTE_ANSWER_MODIFY_AJAX,
data:oParam,method:"post",timeout:5E3,success:this._onSucceedInVoteAnswerModifyAjax.bind(this,$elNewVoteItem,$elNewVoteItemResult,$elOriginVoteItem,$elOriginVoteItemResult,nNewVoteItemNo),error:function(jqXHR,
textStatus,errorThrown){if(textStatus==="timeout")this._onFailInVoteAnswerModifyAjax.bind(this,$elNewVoteItem,$elNewVoteItemResult,$elOriginVoteItem,$elOriginVoteItemResult,naver.kin.pc.common.MESSAGE.TIMEOUT);
else this._onFailInVoteAnswerModifyAjax.bind(this,$elNewVoteItem,$elNewVoteItemResult,$elOriginVoteItem,$elOriginVoteItemResult,naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)};
this._oVoteAnswerModifyAjax=$.ajax(oOption)},
_onSucceedInVoteAnswerModifyAjax:function($elNewVoteItem,$elNewVoteItemResult,$elOriginVoteItem,$elOriginVoteItemResult,nNewVoteItemNo,oResponse){if(!$elNewVoteItem.existElements()||!$elNewVoteItemResult.existElements()||
!$elOriginVoteItem.existElements()||!$elOriginVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}this._oVoteAnswerModifyAjax=null;if(oResponse&&oResponse.isSuccess&&
oResponse.result&&oResponse.result.length>0){var oResult=oResponse.result[0];var nTotalVoteCnt=oResult.totalVoteCnt||0;var oVoteCntInfoMap=oResult.voteCntInfoMap||{};this._nMyAnswerNo=oResult.answerNo||
0;this._setResultList(nTotalVoteCnt,oVoteCntInfoMap,nNewVoteItemNo);this._setTotalVoteCnt(nTotalVoteCnt);this._showResultList($elNewVoteItemResult);this._removeReselectClass();this._showReselectBtn();this._hideReselectCancelBtn();
$elOriginVoteItem.removeClass("is_selected")}else{this._selectVoteItem($elOriginVoteItem,$elOriginVoteItemResult);this._unselectVoteItem($elNewVoteItem,$elNewVoteItemResult);alert(this._MESSAGE.FAILED_TO_ANSWER_REGISTER);
this._bStopEvent=false}},
_onFailInVoteAnswerModifyAjax:function($elNewVoteItem,$elNewVoteItemResult,$elOriginVoteItem,$elOriginVoteItemResult,sErrorMessage){if(!this._oVoteAnswerModifyAjax)this._oVoteAnswerModifyAjax.abort();this._selectVoteItem($elOriginVoteItem,
$elOriginVoteItemResult);this._unselectVoteItem($elNewVoteItem,$elNewVoteItemResult);if(sErrorMessage)alert(sErrorMessage);this._bStopEvent=false},
_voteAnswerDeleteAjax:function($elOriginVoteItem){if(!$elOriginVoteItem){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}var nOriginVoteItemNo=parseInt($elOriginVoteItem.data("itemNo"),
10);if(isNaN(nOriginVoteItemNo)){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}if(this._oVoteAnswerDeleteAjax){alert(this._MESSAGE.VOTE_ANSWER_DELETE_PROCESSING);this._bStopEvent=
false;return}var $elOriginVoteItemResult=$("#voteItemResult"+nOriginVoteItemNo);if(!$elOriginVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);this._bStopEvent=false;return}this._unselectVoteItem($elOriginVoteItem,
$elOriginVoteItemResult);var oParam={dirId:this._nDirId||0,docId:this._nDocId||0,answerNo:this._nMyAnswerNo||0,requestUri:location.href,canceledItemNo:nOriginVoteItemNo};var oOption={url:this._URL.VOTE_ANSWER_DELETE_AJAX,
data:oParam,method:"post",timeout:5E3,success:this._onSucceedInVoteAnswerDeleteAjax.bind(this,$elOriginVoteItem,$elOriginVoteItemResult),error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")this._onFailInVoteAnswerDeleteAjax.bind(this,
$elOriginVoteItem,$elOriginVoteItemResult,naver.kin.pc.common.MESSAGE.TIMEOUT);else this._onFailInVoteAnswerDeleteAjax.bind(this,$elOriginVoteItem,$elOriginVoteItemResult,naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)};
this._oVoteAnswerDeleteAjax=$.ajax(oOption)},
_onSucceedInVoteAnswerDeleteAjax:function($elOriginVoteItem,$elOriginVoteItemResult,oResponse){if(!$elOriginVoteItem.existElements()||!$elOriginVoteItemResult.existElements()){alert(this._MESSAGE.UNKNOWN_VOTE_QUESTION);
this._bStopEvent=false;return}this._oVoteAnswerDeleteAjax=null;if(oResponse&&oResponse.isSuccess&&oResponse.result&&oResponse.result.length>0){var oResult=oResponse.result[0];var nTotalVoteCnt=oResult.totalVoteCnt||
0;this._nMyAnswerNo=0;this._bHasSelectedItem=false;this._setTotalVoteCnt(nTotalVoteCnt);this._hideResultList($elOriginVoteItemResult);this._removeReselectClass();this._hideReselectBtn();this._hideReselectCancelBtn();
$elOriginVoteItem.removeClass("is_selected")}else{this._selectVoteItem($elOriginVoteItem,$elOriginVoteItemResult);alert(this._MESSAGE.FAILED_TO_ANSWER_REGISTER)}this._bStopEvent=false},
_onFailInVoteAnswerDeleteAjax:function($elOriginVoteItem,$elOriginVoteItemResult,sErrorMessage){if(!this._oVoteAnswerDeleteAjax)this._oVoteAnswerDeleteAjax.abort();this._selectVoteItem($elOriginVoteItem,
$elOriginVoteItemResult);if(sErrorMessage)alert(sErrorMessage);this._bStopEvent=false},
_showResultList:function($elVoteItemResult){var $elCheckBtnOfVoteItemListArea=this._$element["voteItemListArea"].find("span._checkBtn");if($elCheckBtnOfVoteItemListArea.existElements())$elCheckBtnOfVoteItemListArea.hide();
var $elCheckBtnOfVoteItemResult=$.getSingle("span._checkBtn",$elVoteItemResult);if($elCheckBtnOfVoteItemResult.existElements()&&!$elCheckBtnOfVoteItemResult.is(":visible"))$elCheckBtnOfVoteItemResult.show();
if(this._$element["voteQuestionArea"].existElements())this._$element["voteQuestionArea"].addClass("result")},
_hideResultList:function($elVoteItemResult){var $elCheckBtnOfVoteItemListArea=this._$element["voteItemListArea"].find("span._checkBtn");if($elCheckBtnOfVoteItemListArea.existElements())$elCheckBtnOfVoteItemListArea.show();
var $elCheckBtnOfVoteItemResult=$.getSingle("span._checkBtn",$elVoteItemResult);if($elCheckBtnOfVoteItemResult.existElements()&&$elCheckBtnOfVoteItemResult.is(":visible")===false)$elCheckBtnOfVoteItemResult.hide();
if(this._$element["voteQuestionArea"].existElements())this._$element["voteQuestionArea"].removeClass("result")},
_setResultList:function(nTotalVoteCnt,oVoteCntInfoMap,nVoteItemNo){if(this._$element["voteItemResultList"].existElements())for(var i=0,len=this._$element["voteItemResultList"].length;i<len;i++){var $elVoteItemResult=
$(this._$element["voteItemResultList"].get(i));if($elVoteItemResult.existElements()){var nItemNo=parseInt($elVoteItemResult.data("itemNo"),10)||0;var nVoteCnt=parseInt(oVoteCntInfoMap[nItemNo],10)||0;var nVotePercent=
this._getVotePercent(nVoteCnt,nTotalVoteCnt);var $elVotePercentList=$("em._votePercent",$elVoteItemResult);if($elVotePercentList.existElements())$elVotePercentList.html(nVotePercent);var $elVoteCountList=
$("p._voteCnt",$elVoteItemResult);if($elVoteCountList.existElements())$elVoteCountList.html(this._nVoteItemCnt<this._VOTE_ITEM_MIN_COUNT_FOR_LIST_TYPE&&nItemNo===nVoteItemNo?'\x3cspan class\x3d"result_check _checkBtn"\x3e\x3c/span\x3e'+
(nVoteCnt<0?0:naver.kin.pc.common.getCommaNumberString(nVoteCnt))+'\uba85 \x3cspan class\x3d"choice"\x3e\uc120\ud0dd\x3c/span\x3e':(nVoteCnt<0?0:naver.kin.pc.common.getCommaNumberString(nVoteCnt))+'\uba85 \x3cspan class\x3d"choice"\x3e\uc120\ud0dd\x3c/span\x3e');
var $elVoteGauge=$.getSingle("span._voteGauage",$elVoteItemResult);if($elVoteGauge.existElements())$elVoteGauge.css(this._nVoteItemCnt<this._VOTE_ITEM_MIN_COUNT_FOR_LIST_TYPE?"height":"width",nVotePercent+
"%")}}},
_setTotalVoteCnt:function(nTotalVoteCnt){if(this._$element["totalVoteCnt"].existElements())this._$element["totalVoteCnt"].html("\ucc38\uc5ec\uc790 "+naver.kin.pc.common.getCommaNumberString(nTotalVoteCnt)+
"\uba85")},
_onClickQuizLayer:function(oEvent){var $el=$(oEvent.target);if($el.hasClass("_close")){this._$element["quizLayer"].hide();oEvent.preventDefault();oEvent.stopPropagation()}},
_onClickCancelVoteItemLayer:function(oEvent){var $el=$(oEvent.target);if($el.hasClass("_close")){this._hideCancelVoteItemLayer();this._bStopEvent=false}else if($el.hasClass("_delete")){this._hideCancelVoteItemLayer();
if(naver.kin.pc.common.checkRos()){oEvent.preventDefault();oEvent.stopPropagation();this._bStopEvent=false;return}var $elOriginVoteItem=$.getSingle("li._voteItemList.on",this._$element["voteItemListArea"]);
if(!$elOriginVoteItem.existElements()){oEvent.preventDefault();oEvent.stopPropagation();this._bStopEvent=false;return}this._voteAnswerDeleteAjax($elOriginVoteItem)}oEvent.preventDefault();oEvent.stopPropagation()},
_showCancelVoteItemLayer:function(){if(this._$element["cancelVoteItemDimmedLayer"].existElements()&&this._$element["cancelVoteItemLayer"].existElements()){this._$element["cancelVoteItemDimmedLayer"].show();
$.getSingle("body").addClass("body_hidden")}},
_hideCancelVoteItemLayer:function(){if(this._$element["cancelVoteItemDimmedLayer"].existElements()){this._$element["cancelVoteItemDimmedLayer"].hide();$.getSingle("body").removeClass("body_hidden")}},
_selectVoteItem:function($elVoteItem,$elVoteItemResult){if($elVoteItem.existElements())$elVoteItem.addClass("on");if($elVoteItemResult.existElements())$elVoteItemResult.addClass("highlight")},
_unselectVoteItem:function($elVoteItem,$elVoteItemResult){if($elVoteItem.existElements())$elVoteItem.removeClass("on");if($elVoteItemResult.existElements())$elVoteItemResult.removeClass("highlight")},
_getVotePercent:function(nVoteCnt,nTotalVoteCnt){if(nVoteCnt<1||nTotalVoteCnt<1)return 0;var nVotePercent=Math.round(nVoteCnt/nTotalVoteCnt*1E3)/10;return nVotePercent===0||nVotePercent===100?nVotePercent:
nVotePercent.toFixed(1)},
_bIsAnswerable:function(){return!this._bHasSelectedItem&&!this._bIsMyQuestion&&this._sState===this._VOTE_QUESTION_STATE.ING}});
window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.Comment=eg.Class({_$element:null,_$template:null,_oOption:null,_bIsProcessing:false,_oPwFilter:null,_oInputEventTimeout:null,_oAjax:null,_oAjaxStatus:null,_oAdditionalQnaListAjax:null,
_COUNT_PER_PAGE:5,_PAGE_PER_SCREEN:10,_TEXTAREA_PLACEHOLDER:"\uac1c\uc778\uc815\ubcf4\ub97c \uacf5\uc720 \ubc0f \uc694\uccad\ud558\uac70\ub098, \uba85\uc608 \ud6fc\uc190, \ubb34\ub2e8 \uad11\uace0, \ubd88\ubc95 \uc815\ubcf4 \uc720\ud3ec\uc2dc \ubaa8\ub2c8\ud130\ub9c1 \ud6c4 \uc0ad\uc81c\ub420 \uc218 \uc788\uc73c\uba70, \uc774\uc5d0 \ub300\ud55c \ubbfc\ud615\uc0ac\uc0c1 \ucc45\uc784\uc740 \uac8c\uc2dc\uc790\uc5d0\uac8c \uc788\uc2b5\ub2c8\ub2e4.",
_MESSAGE:{WAIT_FOR_PROCESSING:"\uc9c4\ud589\uc911\uc778 \uc791\uc5c5\uc774 \uc788\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",WAIT_FOR_LOADING:"\ud398\uc774\uc9c0\ub97c \ub85c\ub529\uc911\uc785\ub2c8\ub2e4. \uc7a0\uc2dc\ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
FAIL_BY_UNKNOWN_REASON:"\uc608\uae30\uce58 \uc54a\uc740 \uc624\ub958\ub85c \uc791\uc5c5\uc5d0 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
INVALID_PARAMETER:"\uc720\ud6a8\ud558\uc9c0 \uc54a\uc740 \ud30c\ub77c\ubbf8\ud130\uc785\ub2c8\ub2e4.",DELETE_COMMENT_CONFIRM:"\ub313\uae00\uc744 \uc0ad\uc81c\ud558\uc2dc\uaca0\uc2b5\ub2c8\uae4c?",EMPTY_CONTENTS:"\ub313\uae00\uc774 \uc785\ub825\ub418\uc9c0 \uc54a\uc558\uc2b5\ub2c8\ub2e4.",
CHECK_LOGIN:"\ub85c\uadf8\uc778 \ud6c4 \uc774\uc6a9\ud558\uc2e4 \uc218 \uc788\uc2b5\ub2c8\ub2e4.",TARGET_COMMENT_ALERT:"\uc548\ub4dc\ub85c\uc774\ub4dc \uc9c0\uc2ddiN\uc571\uc5d0\uc11c\ub9cc \uc0ac\uc6a9 \uac00\ub2a5\ud55c \uae30\ub2a5\uc785\ub2c8\ub2e4.",
ADDITIONAL_QUESTION_COMFIRM:"\ub2f5\ubcc0\uc790\uc5d0\uac8c \ucd94\uac00\ub85c \uc9c8\ubb38\ud558\uace0 \uc2f6\uc73c\uc2e0\uac00\uc694?\n\ud655\uc778\uc744 \ub204\ub974\uba74 \ucd94\uac00\uc9c8\ubb38 \uc4f0\uae30\ud654\uba74\uc73c\ub85c \uc5f0\uacb0\ub429\ub2c8\ub2e4."},
_URL:{COMMENT_LIST_AJAX:"/ajax/detail/commentListAjax.nhn",DELETE_COMMENT_AJAX:"/ajax/detail/commentDeleteAjax.nhn",REGISTER_COMMENT_AJAX:"/ajax/detail/commentRegisterAjax.nhn",REPORT_COMMENT_AJAX:"/ajax/reportComment.nhn"},
construct:function(htOption){this._oOption=htOption||{};this._oAjax={};$(document).on("initCommentListAreaEvent",this._initCommentListAreaEvent.bind(this));$(document).on("addCommentButtonClickEvent",this._initCommentButtonaEvent.bind(this));
$(document).ready(this._onDocumentReady.bind(this))},
_initCommentListAreaEvent:function(oCustomEvent){if(!oCustomEvent)return;var $elCommentListArea=oCustomEvent.commentListArea;if(!$elCommentListArea||!$elCommentListArea.existElements())return;if($elCommentListArea.children().length>
0){$elCommentListArea.show();this._getCommentListAjax($elCommentListArea,1);return}this._initCommentList($elCommentListArea)},
_initCommentButtonaEvent:function(oCustomEvent){if(!oCustomEvent)return;var aWelCommentButtonList=oCustomEvent.commentButtonList;if(!aWelCommentButtonList||aWelCommentButtonList.length<1)return;for(var i=
0,len=aWelCommentButtonList.length;i<len;i++){var $elCommentButton=aWelCommentButtonList[i];if($elCommentButton&&$elCommentButton.existElements())$elCommentButton.on("click",this._onClickCommentBtn.bind(this))}},
_onDocumentReady:function(){this._setElement();this._setEvent();this._initAjaxStatus();if(this._$element["initOpenCommentBtn"].existElements())this._toggleCommentListArea(this._$element["initOpenCommentBtn"])},
_setElement:function(){this._$element={};this._$element["loadingIndicator"]=$("#loadingIndicator");this._$element["commentBtnList"]=$("._commentBtn");this._$element["initOpenCommentBtn"]=this._$element["commentBtnList"].filter(function(index,
el){return $(el).hasClass("_initOpen")});
this._$element["PWLayerArea"]=$("#prohibited_words_layer_area");this._$template={};this._$template["commentListAreaTpl"]=doT.template($("#commentListAreaTpl").html());this._$template["commentListTpl"]=
doT.template($("#commentListTpl").html());this._$template["commentListPagingTpl"]=doT.template($("#commentListPagingTpl").html());this._$template["PWLayer"]=doT.template($("#prohibited_words_layer_tpl").html())},
_setEvent:function(){if(this._$element["commentBtnList"].existElements())this._$element["commentBtnList"].on("click",this._onClickCommentBtn.bind(this));if(this._$element["PWLayerArea"].existElements())this._$element["PWLayerArea"].on("click",
this._onClickPWLayer.bind(this))},
_initAjaxStatus:function(){this._oAjaxStatus={};for(var sKey in this._URL)if(this._URL.hasOwnProperty(sKey)){var sCurrentUrl=this._URL[sKey];this._oAjaxStatus[sCurrentUrl]={isProcessing:false,request:null}}},
_onClickCommentBtn:function(oEvent){oEvent.preventDefault();var $el=$(oEvent.currentTarget);this._toggleCommentListArea($el)},
_toggleCommentListArea:function($el){var $elParent=$el.closest("._contentWrap");if(!$elParent.existElements())return;var $elCommentListArea=$.getSingle("._commentListArea",$elParent);if(!$elCommentListArea.existElements())return;
var $elAdditionalQuestionButton=$.getSingle("a._additionalQuestionBtn",$elParent);if($elAdditionalQuestionButton.existElements())$.getSingle("div._additionalQnaList",$elParent).hide();var bIsNeedToCheckAdditionalQuestion=
!$el.hasClass("is_active")&&$elAdditionalQuestionButton.existElements()&&!$elAdditionalQuestionButton.hasClass("is_active");var bIsActivatedButton=$el.hasClass("is_active");if($elCommentListArea.data("answerNo")!=
0)$elParent.find(".is_active").removeClass("is_active");if(bIsNeedToCheckAdditionalQuestion&&confirm(this._MESSAGE.ADDITIONAL_QUESTION_COMFIRM))this._initAdditionalQuestionArea($elParent);else{if($elCommentListArea.html())if(bIsActivatedButton)$elCommentListArea.hide();
else $elCommentListArea.show();else this._initCommentList($elCommentListArea);$el.toggleClass("is_active",!bIsActivatedButton);naver.kin.pc.common.toggleAriaActiveDescendant($el.parent(),$el.get(0).id);
naver.kin.pc.common.toggleAriaSelected($el.get(0))}},
_initAdditionalQuestionArea:function($elAnswer){if(!$elAnswer||!$elAnswer.existElements())return;var $elAdditionalQuestionButton=$.getSingle("a._additionalQuestionBtn",$elAnswer);if(!$elAdditionalQuestionButton.existElements())return;
var nParentAnswerNo=$elAdditionalQuestionButton.data("answerNo");if(!nParentAnswerNo||nParentAnswerNo<1)return;var $elAdditionalQnaListArea=$.getSingle("div._additionalQnaList",$elAnswer);if(!$elAdditionalQnaListArea.existElements())return;
var $elCommentButton=$.getSingle("a._commentBtn",$elAnswer);if($elCommentButton.existElements())$.getSingle("div._commentListArea",$elAnswer).hide();var bIsActivatedButton=$elAdditionalQuestionButton.hasClass("is_active");
$elAnswer.find(".is_active").removeClass("is_active");if($elAdditionalQnaListArea.html())if(bIsActivatedButton)$elAdditionalQnaListArea.hide();else $elAdditionalQnaListArea.show();else $(document).trigger($.Event("initAdditionalQuestionWriteFormEvent",
{parentAnswerNo:nParentAnswerNo}));$elAdditionalQuestionButton.toggleClass("is_active",!bIsActivatedButton)},
_initCommentList:function($elCommentListArea){if(!this._$element||!this._$template){alert(this._MESSAGE.WAIT_FOR_LOADING);return}if(!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);
return}$elCommentListArea.html(this._$template["commentListAreaTpl"]({"placeholder":this._TEXTAREA_PLACEHOLDER,"isArticleOpen":this._oOption.isArticleOpen}));$elCommentListArea.show();this._setEventForCommentList($elCommentListArea);
this._getCommentListAjax($elCommentListArea,1)},
_setEventForCommentList:function($elCommentListArea){if(!$elCommentListArea.existElements())return;$elCommentListArea.on("click",this._onClickCommentListArea.bind(this));var oTextArea=$.getSingle("textarea",
$elCommentListArea);if(!oTextArea.existElements())return;oTextArea.on("input",this._onInputTextArea.bind(this));oTextArea.on("keyup",this._onInputTextArea.bind(this));oTextArea.on("focus",this._onFocusTextArea.bind(this));
oTextArea.on("blur",this._onBlurTextArea.bind(this))},
_onInputTextArea:function(oEvent){if(this._oInputEventTimeout)clearTimeout(this._oInputEventTimeout);var $elTextArea=oEvent.target;this._oInputEventTimeout=setTimeout(this._refreshCurrentCharCount.bind(this,
$elTextArea),10)},
_onFocusTextArea:function(oEvent){if(!naver.isLogin){$(oEvent.target).blur();if(confirm(this._MESSAGE.CHECK_LOGIN))naver.kin.pc.common.checkLoginAndRedirectToLoginForm();return}var $elTextArea=$(oEvent.target);
if($elTextArea.val()===this._TEXTAREA_PLACEHOLDER)$elTextArea.val("");$elTextArea.removeClass("placeholder")},
_onBlurTextArea:function(oEvent){var $elTextArea=$(oEvent.target);if(!$.trim($elTextArea.val()))this._resetTextArea($elTextArea)},
_refreshCurrentCharCount:function(elTextArea){if(!elTextArea||/textarea/i.test(elTextArea.tagName)===false)return;var $elCurrentCharCount=$.getSingle("span._currentCharCount",elTextArea.parentNode.parentNode);
if(!$elCurrentCharCount.existElements())return;var nTextLength=(elTextArea.value||"").length;$elCurrentCharCount.text(nTextLength+"");this._oInputEventTimeout=null},
_getCommentListAjax:function($elCommentListArea,nPage){if(!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}if($elCommentListArea.hasClass("_commentListArea")===false){var $elParent=
$elCommentListArea.closest("._commentListArea");if(!$elParent.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}$elCommentListArea=$elParent.get(0)}var nAnswerNo=parseInt($elCommentListArea.data("answerNo"),
10)||0;if(!this._oAjax[this._URL.COMMENT_LIST_AJAX])this._oAjax[this._URL.COMMENT_LIST_AJAX]={};if(!this._oAjaxStatus[this._URL.COMMENT_LIST_AJAX][nAnswerNo])this._oAjaxStatus[this._URL.COMMENT_LIST_AJAX][nAnswerNo]=
{isProcessing:false,request:null};if(this._oAjaxStatus[this._URL.COMMENT_LIST_AJAX][nAnswerNo].isProcessing)this._oAjax[this._URL.COMMENT_LIST_AJAX][nAnswerNo].abort();var oAjaxParam={answerNo:nAnswerNo,
count:this._COUNT_PER_PAGE,dirId:this._oOption.dirId||0,docId:this._oOption.docId||0,page:nPage||1};this._oAjax[this._URL.COMMENT_LIST_AJAX][nAnswerNo]=$.ajax({url:this._URL.COMMENT_LIST_AJAX,method:"get",
timeout:5E3,data:oAjaxParam,beforeSend:function(){this._oAjaxStatus[this._URL.COMMENT_LIST_AJAX][nAnswerNo].isProcessing=true}.bind(this),
complete:function(){this._oAjaxStatus[this._URL.COMMENT_LIST_AJAX][nAnswerNo].isProcessing=false}.bind(this),
success:this._onSucceedInCommentListAjax.bind(this,$elCommentListArea,nPage),error:function(jqXHR,textStatus,errorThrown){if(this._oAjax[this._URL.COMMENT_LIST_AJAX][nAnswerNo]){this._oAjax[this._URL.COMMENT_LIST_AJAX][nAnswerNo].abort();
this._oAjax[this._URL.COMMENT_LIST_AJAX][nAnswerNo]=null}if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_onSucceedInCommentListAjax:function($elCommentListArea,nPage,oResponse){if(oResponse&&oResponse.isSuccess){var $elCommentList=$.getSingle("div._commentList",$elCommentListArea);if(!$elCommentList.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);
return}var oResult=oResponse.result;if(oResult){var sThanksMessage=decodeURIComponent($elCommentListArea.data("thanksMessage")||"");$elCommentList.html(this._$template["commentListTpl"]({"currentPage":nPage,
"thanksMessage":sThanksMessage,"list":this._getProcessedCommentList(oResult.commentList)}));var nTotalCount=oResult.totalCnt||0;var $elPagingArea=$.getSingle("div._pagingArea",$elCommentListArea);this._refreshPagingArea($elPagingArea,
nTotalCount,nPage);this._refreshCommentCount($elCommentListArea,nTotalCount)}}else{var sErrorMsg=oResponse&&oResponse.errorMsg||this._MESSAGE.FAIL_BY_UNKNOWN_REASON;alert(sErrorMsg)}},
_getProcessedCommentList:function(aCommentList){if(!aCommentList)return aCommentList;for(var i=0,len=aCommentList.length;i<len;i++){var oCurrentComment=aCommentList[i];oCurrentComment.isDeletable=oCurrentComment.loginUserEqualCommentUser===
true;oCurrentComment.isReportable=oCurrentComment.loginUserEqualCommentUser===false&&(!oCurrentComment.sticker||oCurrentComment===false)}return aCommentList},
_refreshPagingArea:function($elPagingArea,nTotalCount,nPage){if(!$elPagingArea.existElements()||!nTotalCount||!nPage){$elPagingArea.html("");$elPagingArea.hide();return}var nFirstPageOfScreen=Math.floor((nPage-
1)/this._PAGE_PER_SCREEN)*this._PAGE_PER_SCREEN+1;var nLastPage=Math.ceil(nTotalCount/this._COUNT_PER_PAGE);var nLastPageOfScreen=Math.min(nFirstPageOfScreen+this._PAGE_PER_SCREEN-1,nLastPage);if(nFirstPageOfScreen>
nLastPageOfScreen){$elPagingArea.html("");$elPagingArea.hide();return}var aPageList=[];for(var i=nFirstPageOfScreen;i<=nLastPageOfScreen;i++)aPageList.push(i);$elPagingArea.html(this._$template["commentListPagingTpl"]({currentPage:nPage,
pageList:aPageList,showPrevPageBtn:nFirstPageOfScreen>this._PAGE_PER_SCREEN,prevPage:nFirstPageOfScreen-1,showNextPageBtn:nLastPage>nLastPageOfScreen,nextPage:nLastPageOfScreen+1}));$elPagingArea.show()},
_refreshCommentCount:function($elCommentListArea,nTotalCount){if(!$elCommentListArea.existElements()||naver.kin.pc.common.isNumber(nTotalCount)===false)return;var sThanksMessage=decodeURIComponent($elCommentListArea.data("thanksMessage")||
"");if(sThanksMessage)nTotalCount++;var nAnswerNo=parseInt($elCommentListArea.data("answerNo"),10)||0;var $elCommentBtn=$("#cmtstr_"+nAnswerNo);if($elCommentBtn.existElements()){var $elCommentCount=$.getSingle("em._commentCnt",
$elCommentBtn);if($elCommentCount.existElements())if(nTotalCount<1)if(nAnswerNo>0)$elCommentBtn.html('\x3ci class\x3d"icon icon_compose_opinion" aria-hidden\x3d"true"\x3e\x3c/i\x3e\x3cspan class\x3d"button_compose_text _commentText"\x3e\ub313\uae00\x3c/span\x3e');
else{$elCommentCount.html("");$elCommentCount.hide()}else{var sTotalCount="";if(nTotalCount>=1E3)sTotalCount+="999+";else if(nTotalCount>0)sTotalCount+=nTotalCount;$elCommentCount.html(sTotalCount);$elCommentCount.show()}else if(!$elCommentCount.existElements())if(nTotalCount>
0){var sTotalCount="";if(nTotalCount>=1E3)sTotalCount+="999+";else if(nTotalCount>0)sTotalCount+=nTotalCount;$elCommentBtn.html('\x3ci class\x3d"icon icon_compose_opinion" aria-hidden\x3d"true"\x3e\x3c/i\x3e\x3cem class\x3d"button_compose_count _commentCnt"\x3e'+
sTotalCount+"\x3c/em\x3e")}}},
_onClickCommentListArea:function(oEvent){var $elClicked=$(oEvent.target);if(/^(a|button)$/i.test($elClicked.get(0).tagName)===false){var $elParents=$elClicked.parents().map(function(){if(/^(a|button)$/i.test(this.tagName))return $(this)});
if(!$elParents.existElements()){oEvent.preventDefault();oEvent.stopPropagation();return}$elClicked=$elParents.get(0)}var $elCommentListArea=$(oEvent.currentTarget);if($elClicked.hasClass("_pagingBtn")){var nPage=
$elClicked.data("page")||0;this._getCommentListAjax($elCommentListArea,nPage);oEvent.preventDefault();oEvent.stopPropagation()}else if($elClicked.hasClass("_reportBtn")){var nCommentNo=$elClicked.data("commentNo");
this._checkCommentReportableAjax($elCommentListArea,nCommentNo);oEvent.preventDefault();oEvent.stopPropagation()}else if($elClicked.hasClass("_deleteBtn")){if(confirm(this._MESSAGE.DELETE_COMMENT_CONFIRM)){var nCommentNo=
$elClicked.data("commentNo");var nPage=$elClicked.data("page")||0;this._deleteCommentAjax($elCommentListArea,nCommentNo,nPage)}oEvent.preventDefault();oEvent.stopPropagation()}else if($elClicked.hasClass("_registerCommentBtn")){oEvent.preventDefault();
oEvent.stopPropagation();this._requestPwFilterForComment($elCommentListArea)}else if($elClicked.hasClass("_targetCommentWriter")){alert(this._MESSAGE.TARGET_COMMENT_ALERT);oEvent.preventDefault();oEvent.stopPropagation()}},
_checkCommentReportableAjax:function($elCommentListArea,nCommentNo){if(!naver.isLogin){if(confirm(this._MESSAGE.CHECK_LOGIN))naver.kin.pc.common.checkLoginAndRedirectToLoginForm();return}if(naver.kin.pc.common.checkRos())return false;
if(!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}if(this._oAjaxStatus[this._URL.REPORT_COMMENT_AJAX].isProcessing){alert(this._MESSAGE.WAIT_FOR_PROCESSING);return}var sSvc=
this._oOption.svc||"KIN";var oAjaxParam={svc:sSvc,dirId:this._oOption.dirId||0,docId:this._oOption.docId||0,commentNo:nCommentNo||0};if(sSvc==="KIN"){var nAnswerNo=parseInt($elCommentListArea.data("answerNo"),
10)||0;oAjaxParam.answerNo=nAnswerNo}this._oAjax[this._URL.REPORT_COMMENT_AJAX]=$.ajax({url:this._URL.REPORT_COMMENT_AJAX,data:oAjaxParam,method:"post",beforeSend:function(){this._oAjaxStatus[this._URL.REPORT_COMMENT_AJAX].isProcessing=
true}.bind(this),
complete:function(){this._oAjaxStatus[this._URL.REPORT_COMMENT_AJAX].isProcessing=false}.bind(this),
success:function($elCommentListArea,nCommentNo,oResponse){if(oResponse&&oResponse.isSuccess){var sSvc=this._oOption.svc||"KIN";var sPopupUrl="/popup/reportCommentPopup.nhn?svc\x3d"+sSvc+"\x26dirId\x3d"+
(this._oOption.dirId||0)+"\x26docId\x3d"+(this._oOption.docId||0)+"\x26commentNo\x3d"+nCommentNo;if(sSvc==="KIN"){var nAnswerNo=parseInt($elCommentListArea.data("answerNo"),10)||0;sPopupUrl+="\x26answerNo\x3d"+
nAnswerNo}naver.kin.pc.common.popup(sPopupUrl,"reportComment",{width:526,height:550})}else alert(oResponse&&oResponse.errorMsg||naver.kin.pc.common.MESSAGE.ERROR)}.bind(this,$elCommentListArea,nCommentNo),
error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this),
timeout:3E3})},
_deleteCommentAjax:function($elCommentListArea,nCommentNo,nPage){if(!naver.isLogin){if(confirm(this._MESSAGE.CHECK_LOGIN))naver.kin.pc.common.checkLoginAndRedirectToLoginForm();return}if(naver.kin.pc.common.checkRos())return false;
if(!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}if(this._oAjaxStatus[this._URL.DELETE_COMMENT_AJAX].isProcessing){alert(this._MESSAGE.WAIT_FOR_PROCESSING);return}var oAjaxParam=
{answerNo:parseInt($elCommentListArea.data("answerNo"),10)||0,dirId:this._oOption.dirId||0,docId:this._oOption.docId||0,commentNo:nCommentNo||0};this._oAjax[this._URL.DELETE_COMMENT_AJAX]=$.ajax({url:this._URL.DELETE_COMMENT_AJAX+
"?"+$.param(oAjaxParam),method:"post",beforeSend:function(){this._oAjaxStatus[this._URL.DELETE_COMMENT_AJAX].isProcessing=true}.bind(this),
complete:function(){this._oAjaxStatus[this._URL.DELETE_COMMENT_AJAX].isProcessing=false}.bind(this),
success:this._getCommentListAjax.bind(this,$elCommentListArea,nPage),error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this),
timeout:3E3})},
_requestPwFilterForComment:function($elCommentListArea){if(!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}if(!naver.kin.pc.PWFilter){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);
return}if(this._bIsProcessing===true){alert(this._MESSAGE.WAIT_FOR_PROCESSING);return}var oTextArea=$.getSingle("textarea",$elCommentListArea);if(!oTextArea.existElements())return;var sUnescapedContents=
oTextArea.val();oTextArea.text(sUnescapedContents);var sContents=oTextArea.html()||"";if(sContents===this._TEXTAREA_PLACEHOLDER){alert(this._MESSAGE.EMPTY_CONTENTS);return}this._startRegisterProcessing();
this._oPwFilter=new naver.kin.pc.PWFilter({"contents":sContents,"id":"commentText","submitFunction":this._registerCommentAjax.bind(this,sContents,$elCommentListArea),"containProhibitedWordsFunction":this._onContainsPwCallback.bind(this,
$elCommentListArea),"errorCallbackFunction":this._onErrorPwCallback.bind(this)});this._oPwFilter.process()},
_startRegisterProcessing:function(){this._bIsProcessing=true;this._$element["loadingIndicator"].show()},
_finishRegisterProcessing:function(){this._bIsProcessing=false;this._$element["loadingIndicator"].hide();if(this._oPwFilter)this._oPwFilter=null},
_registerCommentAjax:function(sContents,$elCommentListArea){this._finishRegisterProcessing();if(!naver.isLogin){if(confirm(this._MESSAGE.CHECK_LOGIN))naver.kin.pc.common.checkLoginAndRedirectToLoginForm();
return}if(naver.kin.pc.common.checkRos())return false;if(!sContents||!$elCommentListArea.existElements()){alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON);return}if(this._oAjaxStatus[this._URL.REGISTER_COMMENT_AJAX].isProcessing){alert(this._MESSAGE.WAIT_FOR_PROCESSING);
return}var oAjaxParam={answerNo:parseInt($elCommentListArea.data("answerNo"),10)||0,dirId:this._oOption.dirId||0,docId:this._oOption.docId||0,contents:sContents,mdu:this._oOption.mdu||""};this._oAjax[this._URL.REGISTER_COMMENT_AJAX]=
$.ajax({url:this._URL.REGISTER_COMMENT_AJAX,data:oAjaxParam,method:"post",timeout:7E3,beforeSend:function(){this._oAjaxStatus[this._URL.REGISTER_COMMENT_AJAX].isProcessing=true}.bind(this),
complete:function(){this._oAjaxStatus[this._URL.REGISTER_COMMENT_AJAX].isProcessing=false}.bind(this),
success:this._onSucceedInCommentRegisterAjax.bind(this,$elCommentListArea),error:function(jqXHR,textStatus,errorThrown){if(!this._oAjax[this._URL.REGISTER_COMMENT_AJAX]){this._oAjax[this._URL.REGISTER_COMMENT_AJAX].abort();
this._oAjax[this._URL.REGISTER_COMMENT_AJAX]=null}if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_onSucceedInCommentRegisterAjax:function($elCommentListArea,oResponse){if(oResponse&&oResponse.isSuccess){this._resetTextArea($elCommentListArea);this._getCommentListAjax($elCommentListArea,1,oResponse)}else if(oResponse.errorMsg)alert(oResponse.errorMsg);
else alert(naver.kin.pc.common.MESSAGE.ERROR)},
_resetTextArea:function($elTextArea){if(!$elTextArea.existElements())return;if(/^textarea$/i.test($elTextArea.get(0).tagName)===false)$elTextArea=$.getSingle("textarea",$elTextArea);if($elTextArea){$elTextArea.val(this._TEXTAREA_PLACEHOLDER);
$elTextArea.text(this._TEXTAREA_PLACEHOLDER);$elTextArea.addClass("placeholder");var $elCurrentCharCount=$.getSingle("span._currentCharCount",$elTextArea.parent().parent());if($elCurrentCharCount)$elCurrentCharCount.text("0")}},
_onContainsPwCallback:function($elCommentListArea,aProhibitedWords){this._finishRegisterProcessing();if(!aProhibitedWords||aProhibitedWords.length<1)return;setTimeout(function($elCommentListArea,aProhibitedWords){this._showPWLayer($elCommentListArea,
aProhibitedWords)}.bind(this,$elCommentListArea,aProhibitedWords),300)},
_showPWLayer:function($elCommentListArea,aProhibitedWords){if(!aProhibitedWords||aProhibitedWords.length<1||!this._$template["PWLayer"]||!this._$element["PWLayerArea"].existElements())return;var sProhibitedWords=
aProhibitedWords.join(", ");this._$element["PWLayerArea"].html(this._$template["PWLayer"]({"prohibitedWords":sProhibitedWords}));if(!this._$element["PWLayerArea"].is(":visible"))this._$element["PWLayerArea"].show();
this._adjustPositionOfPWLayer()},
_adjustPositionOfPWLayer:function(){if(!this._$element["PWLayerArea"].existElements())return;var $elAdjustPosition=$("._adjustPosition",this._$element["PWLayerArea"]);if($elAdjustPosition.existElements())$elAdjustPosition.css({top:window.pageYOffset+
(document.documentElement.clientHeight-$elAdjustPosition.height())/2+"px",left:(document.documentElement.clientWidth-$elAdjustPosition.width())/2+"px"})},
_hidePWLayer:function(){if(!this._$element["PWLayerArea"].existElements())return;this._$element["PWLayerArea"].html("");if(this._$element["PWLayerArea"].is("visible"))this._$element["PWLayerArea"].hide()},
_onClickPWLayer:function(oEvent){var $el=$(oEvent.target);if(/^img$/i.test($el.get(0).tagName))$el=$el.parent();if(!/^(?:a|button)$/i.test($el.get(0).tagName)){oEvent.preventDefault();oEvent.stopPropagation();
return}if($el.hasClass("_closeBtn")||$el.hasClass("_submitBtn"))this._hidePWLayer();oEvent.preventDefault();oEvent.stopPropagation()},
_onErrorPwCallback:function(){this._finishRegisterProcessing();alert(this._MESSAGE.FAIL_BY_UNKNOWN_REASON)}});
window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.AsideMenu=eg.Class({_$element:null,_$template:null,_messageList:null,_noticeList:null,_oTimer:null,URL:{MESSAGE_LIST_AJAX_URL:"/ajax/user/getMessageList.nhn",NOTICE_LIST_AJAX_URL:"/ajax/common/getNoticeList.nhn",
DELETE_MY_KIN_ACTIVITY:{PUBLIC:"/ajax/removePublicMessage.nhn",PRIVATE:"/ajax/removePrivateMessage.nhn",REALTIME:"/ajax/removeRealtimeMessage.nhn"}},MESSAGE:{NO_MY_MESSAGE:"\x3cdiv class\x3d'popup__nodata'\x3e\x3cp class\x3d'popup__nodata_text'\x3e\ucd5c\uadfc \ub0b4 \uc18c\uc2dd\uc774 \uc5c6\uc2b5\ub2c8\ub2e4.\x3c/p\x3e\x3c/div\x3e",
NO_NOTICE_LIST:"\x3cdiv class\x3d'popup__nodata'\x3e\x3cp class\x3d'popup__nodata_text'\x3e\uacf5\uc9c0\uc0ac\ud56d\uc774 \uc5c6\uc2b5\ub2c8\ub2e4.\x3c/p\x3e\x3c/div\x3e"},INIT_FLAG:{NEW_MESSAGE:false,
NOTICE:false},PAGE_NO:{MESSAGE:0,NOTICE:0},PAGE_PER_COUNT:{MESSAGE:10,NOTICE:10},DATE_FORMAT_LENGTH:11,construct:function(){this._setElement();this._attachEvent();this._loadNoticeList(false)},
_setElement:function(){this._$element={};this._$template={};this._$element["asideMenu"]=$("#asideMenu");this._$element["myMessageButton"]=$.getSingle("a._my_message_button",this._$element["asideMenu"]);
this._$element["noticeButton"]=$.getSingle("a._notice_button",this._$element["asideMenu"]);this._$element["spiButton"]=$.getSingle("a._spi",this._$element["asideMenu"]);this._$element["myMessageLayer"]=
$.getSingle("div._my_message_layer",this._$element["asideMenu"]);this._$element["noticeLayer"]=$.getSingle("div._notice_layer",this._$element["asideMenu"]);this._$element["newNoticeIcon"]=$.getSingle("span._new_notice_icon",
this._$element["asideMenu"]);this._$element["myMessageCount"]=$.getSingle("span._my_message_count",this._$element["asideMenu"]);if(this._$element["myMessageLayer"].existElements())this._$element["myMessageList"]=
$.getSingle("ul._my_message_list",this._$element["myMessageLayer"]);if(this._$element["noticeLayer"].existElements())this._$element["noticeList"]=$.getSingle("ul._notice_list",this._$element["noticeLayer"]);
this._$element["bounceAnimationBoundary"]=$.getSingle("div._bounceAnimationBoundary");this._$template["myMessageItemTpl"]=doT.template($("#myMessageItemTpl").html());this._$template["noticeItemTpl"]=doT.template($("#noticeItemTpl").html())},
_attachEvent:function(){if(this._$element["myMessageButton"].existElements())this._$element["myMessageButton"].on("click",this._clickMessageButton.bind(this));if(this._$element["noticeButton"].existElements())this._$element["noticeButton"].on("click",
this._clickNoticeButton.bind(this));if(this._$element["myMessageLayer"].existElements()){this._$element["myMessageLayer"].on("click",function(oEvent){oEvent.stopPropagation();oEvent.preventDefault()});
this._$element["myMessageList"].on("click","a",this._onClickMyKinActivityArea.bind(this));this._$element["myMessageList"].on("scroll",this._scrollMyMessageList.bind(this))}if(this._$element["noticeLayer"].existElements())this._$element["noticeLayer"].on("click",
this._clickNoticeLayer.bind(this));if(this._$element["spiButton"].existElements())this._$element["spiButton"].on("click",this._clickSpiButton.bind(this));$(document.body).on("click",this._closeAllLayer.bind(this));
$(window).on("scroll",this._onScroll.bind(this))},
_scrollMyMessageList:function(oEvent){var el=oEvent.target;if(Math.round(el.scrollHeight-el.scrollTop)===Math.round(el.clientHeight))if(this._messageList&&this._messageList.length>this.PAGE_NO.MESSAGE*
this.PAGE_PER_COUNT.MESSAGE){this._listMyMessage();this.PAGE_NO.MESSAGE++}},
_listMyMessage:function(){var start=this.PAGE_NO.MESSAGE*this.PAGE_PER_COUNT.MESSAGE+1;var end=start+this.PAGE_PER_COUNT.MESSAGE-1;if(end>this._messageList.length)end=this._messageList.length;var html=
this._$template["myMessageItemTpl"]({list:this._messageList.splice(start,end)});this._$element["myMessageList"].append(html);$("span._alarm_icon",this._$element["myMessageList"]).show()},
_listNotice:function(){var start=this.PAGE_NO.NOTICE*this.PAGE_PER_COUNT.NOTICE+1;var end=start+this.PAGE_PER_COUNT.NOTICE-1;if(end>this._noticeList.length)end=this._noticeList.length;var html=this._$template["noticeItemTpl"]({list:this._noticeList.splice(start,
end)});this._$element["noticeList"].append(html)},
_clickNoticeLayer:function(oEvent){var target=oEvent.target;var $el=$(target);var url="";if(/(?:a)/i.test(target.tagName))url=$el.attr("href");if(/(?:span|div|p)/i.test(target.tagName)){var $elLink=$el.parents("a._notce_link");
url=$elLink.attr("href")}if(url)window.open(url,"_blank");oEvent.stopPropagation();oEvent.preventDefault()},
_closeAllLayer:function(oEvent){if(!this._$element["myMessageLayer"].existElements()||!this._$element["noticeLayer"].existElements())return;if(!this._$element["myMessageLayer"].is(":visible")&&!this._$element["noticeLayer"].is(":visible"))return;
this._$element["myMessageLayer"].hide();this._$element["noticeLayer"].hide();oEvent.stopPropagation();oEvent.preventDefault()},
_closeExceptSelectedLayer:function(layerType){if(layerType=="MESSAGE"){this._$element["noticeLayer"].hide();var $elSpiLayer=this._$element["asideMenu"].find("div._ly_spi");if($elSpiLayer.existElements())$elSpiLayer.hide()}else if(layerType==
"NOTICE"){this._$element["myMessageLayer"].hide();var $elSpiLayer=this._$element["asideMenu"].find("div._ly_spi");if($elSpiLayer.existElements())$elSpiLayer.hide()}else if(layerType=="SHARE"){this._$element["noticeLayer"].hide();
this._$element["myMessageLayer"].hide()}},
_clickMessageButton:function(oEvent){naver.kin.pc.common.checkLoginAndRedirectToLoginForm();if(!naver.isLogin)return false;if(!this._$element["myMessageLayer"].existElements()||!this._$template["myMessageItemTpl"])return;
if(!this.INIT_FLAG.NEW_MESSAGE)this._loadNewMessageList();else this._$element["myMessageLayer"].toggle();this._closeExceptSelectedLayer("MESSAGE");oEvent.stopPropagation();oEvent.preventDefault()},
_loadNewMessageList:function(){$.ajax(this.URL.MESSAGE_LIST_AJAX_URL,{method:"get",timeout:5E3,success:function(res){if(res.isSuccess){var result=res.result;var list=result.kinMessageList;var listSize=
list.length;var html="";this._messageList=list;if(listSize>0){list.forEach(function(notice){if(typeof notice.startTime==="string"&&notice.startTime.length>=this.DATE_FORMAT_LENGTH)notice.startTime=notice.startTime.slice(0,
this.DATE_FORMAT_LENGTH);else notice.startTime=""}.bind(this));
if(listSize>this.PAGE_PER_COUNT.MESSAGE)html=this._$template["myMessageItemTpl"]({list:list.slice(0,this.PAGE_PER_COUNT.MESSAGE)});else html=this._$template["myMessageItemTpl"]({list:list});this._$element["myMessageList"].html(html);
this.PAGE_NO.MESSAGE++}else{this._$element["myMessageList"].hide();this._$element["myMessageLayer"].append(this.MESSAGE.NO_MY_MESSAGE)}$("span._alarm_icon",this._$element["myMessageList"]).show();this._$element["myMessageLayer"].toggle();
this.INIT_FLAG.NEW_MESSAGE=true}}.bind(this),
error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_onClickMyKinActivityArea:function(oEvent){var el=oEvent.currentTarget;if(!el)return;oEvent.stopPropagation();if(el.tagName.toLowerCase()==="a"){var elSpan=$.getSingle("span",el);if(elSpan.existElements())el=
elSpan.get(0)}var $el=$(el);naver.kin.pc.common.nClicks("end*l.myflist","","",oEvent,"");if($el.hasClass("_ros")&&naver.kin.pc.common.checkRos())return;if($el.hasClass("_btn_remove_alrimi_private")){var nMessageNo=
$el.attr("no");var oParam={"no":nMessageNo};this._deleteMyKinActivity(oParam,$el,"PRIVATE")}else if($el.hasClass("_btn_remove_alrimi_public")){var nMessageNo=$el.attr("no");var oParam={"no":nMessageNo};
this._deleteMyKinActivity(oParam,$el,"PUBLIC")}else if($el.hasClass("_btn_remove_one2one")){var nDirId=$el.attr("dirid");var nDocId=$el.attr("docid");var oParam={"category":"one2one","dirId":nDirId,"docId":nDocId};
this._deleteMyKinActivity(oParam,$el,"REALTIME")}else if($el.hasClass("_btn_remove_one2one_return")){var nDirId=$el.attr("dirid");var nDocId=$el.attr("docid");var oParam={"category":"one2oneReturn","dirId":nDirId,
"docId":nDocId};this._deleteMyKinActivity(oParam,$el,"REALTIME")}else if($el.hasClass("_btn_remove_change_activity")){var sRemoveType=$el.attr("removetype");var oParam={"category":"activity","type":sRemoveType};
this._deleteMyKinActivity(oParam,$el,"REALTIME")}else if($el.hasClass("_btn_remove_editor_alrimi")){var nDocId=$el.attr("docid");var nNboardId=$el.attr("nboardid");var oParam={"category":"editorNotice",
"docId":nDocId,"nboardId":nNboardId};this._deleteMyKinActivity(oParam,$el,"REALTIME")}},
_deleteMyKinActivity:function(oParam,$elClicked,messageType){$.ajax(this.URL.DELETE_MY_KIN_ACTIVITY[messageType],{method:"post",timeout:3E3,data:oParam,success:function(oRes){if(oRes.result=="SUCCESS"){if(!$elClicked.existElements())return;
var $elParentList=$elClicked.parents("li");if(!$elParentList.existElements()||$elParentList.length<1)return;var $elCurrentLi=$($elParentList[0]);if(!$elCurrentLi.existElements())return;$elCurrentLi.remove()}}.bind(this),
error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_clickNoticeButton:function(oEvent){if(!this._$element["noticeLayer"].existElements()||!this._$template["noticeItemTpl"])return;if(!this.INIT_FLAG.NOTICE)this._loadNoticeList(true);else this._$element["noticeLayer"].toggle();
this._closeExceptSelectedLayer("NOTICE");oEvent.stopPropagation();oEvent.preventDefault()},
_loadNoticeList:function(bShow){$.ajax(this.URL.NOTICE_LIST_AJAX_URL,{method:"get",timeout:5E3,success:function(res){if(res.isSuccess){var result=res.result;var list=result.result.noticeList;var hasNewNoticeExist=
result.result.newNoticeExist;var listSize=list.length;var html="";this._noticeList=list;if(hasNewNoticeExist)this._$element["newNoticeIcon"].show();if(listSize>0){list.forEach(function(notice){if(typeof notice.startTime===
"string"&&notice.startTime.length>=this.DATE_FORMAT_LENGTH)notice.startTime=notice.startTime.slice(0,this.DATE_FORMAT_LENGTH);else notice.startTime=""}.bind(this));
html=this._$template["noticeItemTpl"]({list:list.slice(0,this.PAGE_PER_COUNT.NOTICE)});this._$element["noticeList"].html(html);this.PAGE_NO.NOTICE++}else{this._$element["noticeList"].hide();this._$element["noticeLayer"].append(this.MESSAGE.NO_NOTICE_LIST)}}if(bShow)this._$element["noticeLayer"].toggle();
this.INIT_FLAG.NOTICE=true}.bind(this),
error:function(jqXHR,textStatus,errorThrown){if(textStatus==="timeout")alert(naver.kin.pc.common.MESSAGE.TIMEOUT);else alert(naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_clickSpiButton:function(oEvent){this._closeExceptSelectedLayer("SHARE");oEvent.stopPropagation();oEvent.preventDefault()},
_onScroll:function(){if(this._oTimer)clearTimeout(this._oTimer);this._oTimer=setTimeout(function(){var nPadding=52;var nScrollTop=$(document).scrollTop();var nBoundary=this._$element["bounceAnimationBoundary"].offset().top-
this._$element["asideMenu"].height()-nPadding;var nTop=Math.min(Math.max(nPadding,nScrollTop),nBoundary-nPadding);if(nBoundary<nScrollTop&&!this._$element["asideMenu"].hasClass("_bounceAnimation"))this._$element["asideMenu"].addClass("_bounceAnimation").addClass("bounce");
else this._$element["asideMenu"].removeClass("_bounceAnimation").removeClass("bounce");this._$element["asideMenu"].stop().animate({top:nTop},100)}.bind(this),30)}});
window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.BottomQnaList=eg.Class({_COUNT_PER_PAGE:4,_$element:null,_$template:null,_sDetailQnaListAreaId:null,_sDetailQnaListAjaxUrl:null,_oDetailQnaListAjaxData:null,_sDetailQnaListTemplateId:null,
_fnGetProcessedResult:null,construct:function(oOption){if(!oOption)return;this._sDetailQnaListAreaId=oOption.sDetailQnaListAreaId;this._sDetailQnaListAjaxUrl=oOption.sDetailQnaListAjaxUrl;this._oDetailQnaListAjaxData=
oOption.oDetailQnaListAjaxData||{};this._sDetailQnaListTemplateId=oOption.sDetailQnaListTemplateId;this._fnGetProcessedResult=oOption.fnGetProcessedResult||function(oResult){return oResult};
if(!this._sDetailQnaListAreaId||!this._sDetailQnaListTemplateId||!this._sDetailQnaListAjaxUrl)return;this._initAjaxOption();this._setElement();this._setEvent();this._getDetailQnaList()},
_initAjaxOption:function(){this._oAjaxOption={url:this._sDetailQnaListAjaxUrl,data:this._oDetailQnaListAjaxData,method:"get",timeout:5E3,dataType:"json",cache:false,success:function(oResponse){if(oResponse&&
oResponse.isSuccess){var oResult=oResponse.result||{};oResult.countPerPage=this._COUNT_PER_PAGE;oResult.totalCount=(oResult.list||[]).length;if(oResult.totalCount<1){this._$element["detailQnaListComponent"].hide();
return}oResult=this._fnGetProcessedResult(oResult);if(!oResult){this._$element["detailQnaListComponent"].hide();return}var sHtml=this._$template["detailQnaList"](oResult);if(!sHtml){this._$element["detailQnaListComponent"].hide();
return}this._$element["detailQnaListArea"].append(sHtml);this._$element["detailQnaList"]=this._$element["detailQnaListArea"].find("div._detailQnaList");var paginatorSize=oResult.totalCount/oResult.countPerPage;
var sPaginatorHtml="";for(var i=0;i<paginatorSize;i++)sPaginatorHtml+=this._$template["qnaListPaginatorTpl"]({index:i,elementId:this._sDetailQnaListAreaId});this._$element["qnaListPaginatorWrap"].find("._paginatorInner").append(sPaginatorHtml);
this._$element["paginator"]=this._$element["qnaListPaginatorWrap"].find("._paginator");this._$element["paginator"].on("click",function(oEvent){var $elTarget=$(oEvent.target);var nIndex=$elTarget.data("index");
var $elSelectedPaginator=$(this._$element["paginator"].get(nIndex));var $elSelectItem=$(this._$element["detailQnaList"].get(nIndex));this._$element["visibleDetailQnaList"].hide();this._$element["visibleDetailQnaList"]=
$elSelectItem;$elSelectItem.show();this._addPaginatorActiveState($elSelectedPaginator);this._setPrevButtonAndNextButton();oEvent.stopPropagation();oEvent.preventDefault()}.bind(this));
this._$element["detailQnaListAreaWrapper"].show();this._$element["qnaListPaginatorWrap"].show();this._$element["visibleDetailQnaList"]=this._$element["detailQnaListArea"].find("div._detailQnaList:visible");
this._addPaginatorActiveState($(this._$element["paginator"].get(0)));this._setPrevButtonAndNextButton()}else this._$element["detailQnaListComponent"].hide()}.bind(this),
error:function(){this._$element["detailQnaListComponent"].hide()}.bind(this)}},
_setElement:function(){this._$element={};this._$element["detailQnaListComponent"]=$("._detailQnaListComponent");this._$element["detailQnaListAreaWrapper"]=$("#"+this._sDetailQnaListAreaId);this._$element["detailQnaListArea"]=
$.getSingle("div._detailQnaListArea",this._$element["detailQnaListAreaWrapper"]);this._$element["prevButton"]=$.getSingle("a._prevButton",this._$element["detailQnaListAreaWrapper"]);this._$element["nextButton"]=
$.getSingle("a._nextButton",this._$element["detailQnaListAreaWrapper"]);this._$element["qnaListPaginatorWrap"]=$.getSingle("div._qnaListPaginator",this._$element["detailQnaListAreaWrapper"]);this._$template=
{};this._$template["detailQnaList"]=doT.template($("#"+this._sDetailQnaListTemplateId).html());this._$template["qnaListPaginatorTpl"]=doT.template($("#qnaListPaginatorTpl").html())},
_setEvent:function(){this._$element["prevButton"].on("click",function(oEvent){var $elPrevDetailQnaList=this._$element["visibleDetailQnaList"].prev("div._detailQnaList");var $elPrevPaginator=this._$element["visiblePaginator"].prev("a._paginator");
$elPrevDetailQnaList.show();this._$element["visibleDetailQnaList"].hide();this._$element["visibleDetailQnaList"]=$elPrevDetailQnaList;this._setPrevButtonAndNextButton();this._addPaginatorActiveState($elPrevPaginator);
oEvent.preventDefault();oEvent.stopPropagation()}.bind(this));
this._$element["nextButton"].on("click",function(oEvent){var $elNextDetailQnaList=this._$element["visibleDetailQnaList"].next("div._detailQnaList");var $elNextPaginator=this._$element["visiblePaginator"].next("a._paginator");
$elNextDetailQnaList.show();this._$element["visibleDetailQnaList"].hide();this._$element["visibleDetailQnaList"]=$elNextDetailQnaList;this._setPrevButtonAndNextButton();this._addPaginatorActiveState($elNextPaginator);
oEvent.preventDefault();oEvent.stopPropagation()}.bind(this))},
_getDetailQnaList:function(){$.ajax(this._oAjaxOption)},
_addPaginatorActiveState:function($elPaginator){this._$element["paginator"].removeClass("is_active");this._$element["paginator"].attr({"aria-selected":"false"});$elPaginator.addClass("is_active");$elPaginator.attr({"aria-selected":"true"});
this._$element["visiblePaginator"]=$elPaginator},
_setPrevButtonAndNextButton:function(){if(this._$element["visibleDetailQnaList"].next("div._detailQnaList").existElements())this._$element["nextButton"].show();else this._$element["nextButton"].hide();
if(this._$element["visibleDetailQnaList"].prev("div._detailQnaList").existElements())this._$element["prevButton"].show();else this._$element["prevButton"].hide()}});
window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};naver.kin.pc.detail=naver.kin.pc.detail||{};
naver.kin.pc.detail.TicketLayer=eg.Class({_$element:null,_$template:null,construct:function(htOption){this._oOption=htOption||{};this._setElement();this._setEvent();if(this._oOption.showLayer&&this._$element["ticketCreationLayer"])this.showLayer();
$(window).on("unload",this._destroy.bind(this))},
_setElement:function(){this._$element={};this._$template={};this._$element["ticketCreationLayer"]=$("#ticketCreationLayer");if(this._$element["ticketCreationLayer"].existElements()){this._$element["todayNotShowCheckBox"]=
$.getSingle("input._todayNotShow",this._$element["ticketCreationLayer"]);this._$element["closeBtn"]=$.getSingle("a._close",this._$element["ticketCreationLayer"]);this._$element["rouletteHomeBtn"]=$.getSingle("a._rouletteHome",
this._$element["ticketCreationLayer"]);this._$element["wordChainHomeBtn"]=$.getSingle("a._wordChainHome",this._$element["ticketCreationLayer"]);this._$template["allTicketCreationLayer"]=doT.template($("#allTicketCreationLayerTpl").html());
this._$template["rouletteTicketCreationLayer"]=doT.template($("#rouletteTicketCreationLayerTpl").html());this._$template["wordChainTicketCreationLayer"]=doT.template($("#wordChainTicketCreationLayerTpl").html())}},
_setEvent:function(){if(this._$element["closeBtn"].existElements())this._$element["closeBtn"].on("click",this._onClickCloseBtn.bind(this));if(this._$element["rouletteHomeBtn"].existElements())this._$element["rouletteHomeBtn"].on("click",
this._onClickRouletteHomeBtn.bind(this));if(this._$element["wordChainHomeBtn"].existElements())this._$element["wordChainHomeBtn"].on("click",this._onClickWordChainHomeBtn.bind(this))},
showLayer:function(oResult){cookie.erase("detail_feedback");if(this._checkedLayerNotOpenToday())return;if(oResult){var sHtml="";if(oResult.rouletteTicketCreated&&oResult.wordChainTicketCreated)sHtml=this._$template["allTicketCreationLayer"]();
else if(oResult.rouletteTicketCreated)sHtml=this._$template["rouletteTicketCreationLayer"]();else if(oResult.wordChainTicketCreated)sHtml=this._$template["wordChainTicketCreationLayer"]();if(sHtml){this._$element["ticketCreationLayer"].append(sHtml);
this._$element["todayNotShowCheckBox"]=$.getSingle("input._todayNotShow",this._$element["ticketCreationLayer"]);this._$element["closeBtn"]=$.getSingle("a._close",this._$element["ticketCreationLayer"]);
this._$element["rouletteHomeBtn"]=$.getSingle("a._rouletteHome",this._$element["ticketCreationLayer"]);this._$element["wordChainHomeBtn"]=$.getSingle("a._wordChainHome",this._$element["ticketCreationLayer"]);
this._setEvent()}}$(document.body).addClass("body_hidden");this._$element["ticketCreationLayer"].show()},
_onClickCloseBtn:function(){var elCheckBoxDoNotShowToday=this._$element["todayNotShowCheckBox"].get(0);if(elCheckBoxDoNotShowToday&&elCheckBoxDoNotShowToday.checked)this._setTimeLocalStorage();if(this._$element["ticketCreationLayer"]){this._$element["ticketCreationLayer"].hide();
$(document.body).removeClass("body_hidden")}},
_onClickRouletteHomeBtn:function(){var elCheckBoxDoNotShowToday=this._$element["todayNotShowCheckBox"].get(0);if(elCheckBoxDoNotShowToday&&elCheckBoxDoNotShowToday.checked)this._setTimeLocalStorage();location.href=
"/roulette/home.nhn"},
_onClickWordChainHomeBtn:function(){var elCheckBoxDoNotShowToday=this._$element["todayNotShowCheckBox"].get(0);if(elCheckBoxDoNotShowToday&&elCheckBoxDoNotShowToday.checked)this._setTimeLocalStorage();
location.href=mobileDomainWithProtocol+"/mobile/wordchain/home.nhn"},
_setTimeLocalStorage:function(){var oLocalStorage=naver.kin.pc.common.LocalStorage;if(oLocalStorage.isSupportLocalStorage())oLocalStorage.setItem(oLocalStorage.KEY.PCWEB_TICKET_LAYER_CLOSED_TIME,(new Date).getTime())},
_checkedLayerNotOpenToday:function(){var oLocalStorage=naver.kin.pc.common.LocalStorage;var sItem=oLocalStorage.getItem(oLocalStorage.KEY.PCWEB_TICKET_LAYER_CLOSED_TIME);var nDiff=(new Date).getTime()-
parseInt(sItem||0,10);return nDiff<864E5},
_destroy:function(){if(this._$element["closeBtn"].existElements())this._$element["closeBtn"].off("click");if(this._$element["rouletteHomeBtn"].existElements())this._$element["rouletteHomeBtn"].on("click");
if(this._$element["wordChainHomeBtn"].existElements())this._$element["wordChainHomeBtn"].on("click");$(document).off("kiniclose");this._$element=this._$template=null}});
window.naver=window.naver||{};naver.kin=naver.kin||{};naver.kin.pc=naver.kin.pc||{};
naver.kin.pc.SocialPlugIn=eg.Class({_COUNT_PER_PAGE:4,_URL:{GET_ANSWER_LIKE_COUNT_AJAX:"/ajax/getAnswerLikeCountAjax.nhn",GET_ANSWER_LIKE_USER_LIST_AJAX:"/ajax/getAnswerLikeUserListAjax.nhn"},_MESSAGE:{WAIT_FOR_PROCESSING:"\uc9c4\ud589\uc911\uc778 \uc791\uc5c5\uc774 \uc788\uc2b5\ub2c8\ub2e4. \uc7a0\uc2dc\ud6c4 \ub2e4\uc2dc \uc2dc\ub3c4\ud574\uc8fc\uc138\uc694.",
FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN:"\uc608\uae30\uce58 \uc54a\uc740 \uc624\ub958\ub85c \ucee8\ud150\uce20\ub97c \uacf5\uac10\ud55c \ud68c\uc6d0 \ubaa9\ub85d \uc870\ud68c\uc5d0 \uc2e4\ud328\ud558\uc600\uc2b5\ub2c8\ub2e4.",
FAILED_TO_GET_LIKE_USER_LIST_EMPTY:"\ud574\ub2f9 \ucee8\ud150\uce20\ub97c \uacf5\uac10\ud55c \ud68c\uc6d0\uc774 \uc5c6\uc2b5\ub2c8\ub2e4."},_DEFAULT_REACTION_INIT_OPTION:{type:"face",domain:"https://common.like.naver.com",
staticDomain:null,dependentLibrary:"jquery",language:"ko",authType:"nid",cssId:null,moduleClassname:"_reactionModule",iconToggleClassname:["on","off"],contentCountPerOnceRequest:20,maxCount:9999,faceButtonMaxIconCount:2,
isZeroFace:true,isMobile:false,isDuplication:false,isHiddenIcon:false,isHiddenCount:false,isHiddenZeroCount:false,isUsedLabelAsZeroCount:false,isDebugMode:false,callback:null,clicklog:null},_oOption:null,
_$element:null,_$template:null,_sReactionModuleClassName:null,_sButtonClassToBeFocused:null,_bIsAjaxProcessing:false,construct:function(oOption){if(!oOption)return;this._oOption=oOption;this._oOption.linkUrl=
this._oOption.linkUrl||location.href;this._oOption.likeItDomain=this._oOption.likeItDomain||"https://common.like.naver.com";this._oOption.socialPlugInDomain=this._oOption.socialPlugInDomain||"https://ssl.pstatic.net/spi";
var oCurrentDate=new Date,yyyy=oCurrentDate.getFullYear(),mm=oCurrentDate.getMonth()+1,dd=oCurrentDate.getDate();this._oOption.sScriptVersion=yyyy+(mm<10?"0"+mm:mm)+(dd<10?"0"+dd:dd);this._setElement();
this._setEvent();if(document.readyState===undefined||document.readyState==="complete")this._initializeOnLoad();else $(window).on("load",this._initializeOnLoad.bind(this))},
_setElement:function(){this._$element={};this._$element["likeReactionModule"]=$("div._reactionModule");this._$element["likeUserListBtn"]=$("a._likeUserListBtn");this._$element["socialPluginArea"]=$("#socialPluginArea");
this._$element["socialPlugInButton"]=$("._spi");this._$element["loadingIndicator"]=$("#loadingIndicator");this._$template={};this._$template["answerLikeUserLayerTpl"]=doT.template($("#answerLikeUserLayerTpl").html());
this._$template["answerLikeCountAreaTpl"]=doT.template($("#answerLikeCountAreaTpl").html());this._$template["answerLikeUserListTpl"]=doT.template($("#answerLikeUserListTpl").html());this._$template["answerLikeUserListPagingTpl"]=
doT.template($("#answerLikeUserListPagingTpl").html())},
_setEvent:function(){if(this._$element["likeUserListBtn"]&&this._$element["likeUserListBtn"].existElements())this._$element["likeUserListBtn"].on("click",this._onClickLikeUserListButton.bind(this));$(document).on("refreshSocialPlugInEvent",
function(){this.refresh()}.bind(this));
$(document.body).on("click",this._onClickDocumentBody.bind(this));$(window).on("resize",this._onResizeWindow.bind(this))},
_initializeOnLoad:function(){if(this._$element["likeReactionModule"].existElements())this._lazyLoadILikeItScript();if(this._$element["socialPluginArea"].existElements())this._lazyLoadSocialPlugInScript()},
_lazyLoadILikeItScript:function(){if(this._$element["likeReactionModule"].existElements()){var sReactionScriptUrl=this._oOption.likeItDomain+"/static/js/reaction/dist/reaction.min.js?v\x3d"+this._oOption.sScriptVersion;
if(this._oOption.reactionInitOption&&this._oOption.reactionInitOption.staticDomain)sReactionScriptUrl=this._oOption.reactionInitOption.staticDomain+"/js/reaction/dist/reaction.min.js?v\x3d"+this._oOption.sScriptVersion;
$.cachedScript(sReactionScriptUrl).done(this._onLoadReactionScript.bind(this))}},
_onLoadReactionScript:function(){if(typeof reaction==="undefined")return;var oReactionInitOption=this._DEFAULT_REACTION_INIT_OPTION;var oAdditionalReactionInitOption=this._oOption.reactionInitOption;if(typeof oAdditionalReactionInitOption===
"object")for(var sKey in oAdditionalReactionInitOption)if(oAdditionalReactionInitOption.hasOwnProperty(sKey))oReactionInitOption[sKey]=oAdditionalReactionInitOption[sKey];this._sReactionModuleClassName=
oReactionInitOption.moduleClassname;this._setLikeReactionCallback(oReactionInitOption);reaction.init(oReactionInitOption)},
_setLikeReactionCallback:function(oReactionInitOption){if(!oReactionInitOption.callback)oReactionInitOption.callback={};if(typeof oReactionInitOption.callback.updated!=="function")oReactionInitOption.callback.updated=
function(oParam){if(!oParam)return;var aLikeContentsList=oParam.contents||[];var aElLikeModuleList=oParam.targets||[];for(var i=0,len=aLikeContentsList.length;i<len;i++){var $elLikeModule=$(aElLikeModuleList[i]);
if(!$elLikeModule.existElements())return;if(!$elLikeModule.is(":visible")){var sDisplay=$elLikeModule.data("display")||"inline-block";$elLikeModule.css("display",sDisplay)}var aReactions=aLikeContentsList[i].reactions||
[];if(this._getTotalReactionCount(aReactions)>0){var $elLikeUserListBtn=$.getSingle("a._likeUserListBtn",$elLikeModule.parent());$elLikeUserListBtn.css("display","inline-block")}}}.bind(this);
if(typeof oReactionInitOption.callback.updateParent!=="function")oReactionInitOption.callback.updateParent=function(oParam){}.bind(this);
if(typeof oReactionInitOption.callback.clicked!=="function")oReactionInitOption.callback.clicked=function(oParam){if(!oParam)return;var aElTargets=oParam.targets;if(aElTargets&&aElTargets.length>0)this._refreshLikeCount($(aElTargets[0]));
if(typeof this._oOption.fnReactionCallback!=="function")return;var oLikeContentsInfo=oParam.content;if(!oLikeContentsInfo)return;var sContentsId=oLikeContentsInfo.contentsId;var oLikeContentsIdInfo=naver.kin.pc.common.getLikeContentsInfo(sContentsId);
if(!oLikeContentsIdInfo)return;var sReactionType=oLikeContentsInfo.reactionType;if(!sReactionType)return;var sActionEvent=oLikeContentsInfo.isReacted?"add":"delete";this._oOption.fnReactionCallback({contentsId:sContentsId,
parentContentsId:oLikeContentsIdInfo.gdId,answerNo:oLikeContentsIdInfo.answerNo,reactionType:sReactionType,actionEvent:sActionEvent})}.bind(this);
if(typeof oReactionInitOption.clicklog!=="function")oReactionInitOption.clicklog=function(elTarget,sNclicksCode,oEvent){if(!elTarget||!sNclicksCode)return;naver.kin.pc.common.nClicks(sNclicksCode,"","",
oEvent)}.bind(this)},
_getTotalReactionCount:function(aReaction){if(!aReaction||aReaction instanceof Array===false)return 0;var nTotalReactionCount=0;for(var i=0,len=aReaction.length;i<len;i++)nTotalReactionCount+=aReaction[i].count||
0;return nTotalReactionCount},
_refreshLikeCount:function($elAnswerLikeArea){if(!$elAnswerLikeArea.existElements())return;if(!$elAnswerLikeArea.hasClass("_answerLikeArea"))$elAnswerLikeArea=$elAnswerLikeArea.closest("._answerLikeArea");
var $elLikeUserListBtn=$.getSingle("a._likeUserListBtn",$elAnswerLikeArea);if(!$elLikeUserListBtn.existElements())return;var $elReactionCount=$.getSingle("a._face span._count",$elAnswerLikeArea);if(!$elReactionCount.existElements())return;
var nTotalReactionCount=parseInt($elReactionCount.text(),10)||0;if(nTotalReactionCount>0)$elLikeUserListBtn.css("display","inline-block");else $elLikeUserListBtn.hide()},
_lazyLoadSocialPlugInScript:function(){var oServiceSetting=this._oOption.serviceSetting||{};var sLinkUrl=this._oOption.linkUrl;var sTitle=this._oOption.title||"Unknown";var sSourceTitle=this._oOption.sourceTitle||
"Unknown";var sSourceContents=null;var oInitializeData={evKey:"kin",servicName:"\uc9c0\uc2ddiN",url:sLinkUrl,title:sTitle,me:{display:"off"},mail:{display:oServiceSetting["mail"]==="off"?"off":"on",url:sLinkUrl,
title:sTitle,srvid:"social",srvurl:""},blog:{display:oServiceSetting["blog"]==="off"?"off":"on",url:sLinkUrl,title:sTitle,blogId:"naver",sourceType:"111",sourceTitle:sSourceTitle,sourceContents:sSourceContents,
sourceForm:2},cafe:{display:oServiceSetting["cafe"]==="off"?"off":"on",url:sLinkUrl,title:sTitle,sourceType:"111",sourceTitle:sSourceTitle,sourceContents:sSourceContents,sourceForm:2},memo:{display:oServiceSetting["memo"]===
"off"?"off":"on",url:sLinkUrl,title:sTitle},calendar:{display:oServiceSetting["calendar"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},bookmark:{display:oServiceSetting["bookmark"]==="off"?"off":"on",
url:sLinkUrl,title:sTitle},copyurl:{display:oServiceSetting["copyurl"]==="off"?"off":"on",url:sLinkUrl},twitter:{display:oServiceSetting["twitter"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},facebook:{display:oServiceSetting["facebook"]===
"off"?"off":"on",url:sLinkUrl,title:sTitle},band:{display:oServiceSetting["band"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},line:{display:oServiceSetting["line"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},
kakaotalk:{display:oServiceSetting["kakaotalk"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},kakaostory:{display:oServiceSetting["kakaostory"]==="off"?"off":"on",url:sLinkUrl,title:sTitle},score:{display:"off"}};
if(typeof window.initializeSocialPlugin==="undefined")window.initializeSocialPlugin=oInitializeData||null;var sSocialPlugInScriptUrl=this._oOption.socialPlugInDomain+"/js/release/ko_KR/splugin.js?"+this._oOption.sScriptVersion;
$.cachedScript(sSocialPlugInScriptUrl).done(this._onLoadSocialPlugInScript.bind(this))},
_onLoadSocialPlugInScript:function(){if(typeof SocialPlugIn_Core=="undefined")return;window.__splugin=SocialPlugIn_Core({"evKey":"kin","serviceName":"\uc9c0\uc2ddiN","dimmed":"default","onClick":function(button){},
"onShow":function(){},
"onHide":function(){},
"onResize":function(event){},
"onRotate":function(event){}})},
refresh:function(bIsForceUpdate){this._$element["likeUserListBtn"].off("click");this._$element["likeReactionModule"]=$("div._reactionModule");this._$element["likeUserListBtn"]=$("a._likeUserListBtn");this._$element["likeUserListBtn"].on("click",
this._onClickLikeUserListButton.bind(this));if(window.__splugin)window.__splugin.init();if(typeof reaction!=="undefined"&&reaction.instance&&typeof reaction.instance.update==="function")if(bIsForceUpdate===
true)reaction.instance.update(null,true);else reaction.instance.update()},
_onClickLikeUserListLayer:function(oEvent){var $elTarget=$(oEvent.target);if($elTarget.get(0).tagName!=="a")$elTarget=$elTarget.closest("a");if(!$elTarget.existElements()){oEvent.stopPropagation();oEvent.preventDefault();
return}var $elLikeUserListLayer=$(oEvent.currentTarget);var nLikeAnswerNo=parseInt($elLikeUserListLayer.data("answerNo"),10);if($elTarget.hasClass("_closeBtn")){if($elLikeUserListLayer.existElements())this._closeLikeUserListLayer(nLikeAnswerNo,
$elLikeUserListLayer);oEvent.stopPropagation();oEvent.preventDefault()}else if($elTarget.hasClass("_reactionTypeBtn")){if(nLikeAnswerNo>0){var sReactionType=$elTarget.data("reactionType")||null;this._initLikeUserListLayer(nLikeAnswerNo,
sReactionType)}oEvent.stopPropagation();oEvent.preventDefault()}else if($elTarget.hasClass("_prevPageBtn")){if(nLikeAnswerNo>0){var sReactionType=$elTarget.data("reactionType")||null;var nPage=$elTarget.data("page")||
1;this._sButtonClassToBeFocused="._prevPageBtn";this._likeUserListAjax($elLikeUserListLayer,"prev",sReactionType,nLikeAnswerNo,nPage)}oEvent.stopPropagation();oEvent.preventDefault()}else if($elTarget.hasClass("_nextPageBtn")){if(nLikeAnswerNo>
0){var sReactionType=$elTarget.data("reactionType")||null;var nPage=$elTarget.data("page")||1;this._sButtonClassToBeFocused="._nextPageBtn";this._likeUserListAjax($elLikeUserListLayer,"next",sReactionType,
nLikeAnswerNo,nPage)}oEvent.stopPropagation();oEvent.preventDefault()}},
_closeLikeUserListLayer:function(nLikeAnswerNo,$elLikeUserListLayer){$elLikeUserListLayer.hide();$elLikeUserListLayer.prop("aria-hidden","true");var $elLikeUserListBtn=$.getSingle("#answerLikeArea_"+nLikeAnswerNo+
" a._likeUserListBtn");$elLikeUserListBtn.focus();$elLikeUserListBtn.prop("aria-pressed","false")},
_onKeydownLikeUserListLayer:function(oEvent){var $elLikeUserListLayer=$(oEvent.currentTarget);if(!$elLikeUserListLayer.existElements())return;var $elTarget=$(oEvent.target);var nKeyCode=oEvent.keyCode||
oEvent.which;if($elTarget.hasClass("_firstReactionBtn")&&oEvent.shiftkey&&nKeyCode===9){$.getSingle("._closeBtn",$elLikeUserListLayer).focus();oEvent.stopPropagation();oEvent.preventDefault()}else if($elTarget.hasClass("_closeBtn"))if(!oEvent.shiftkey&&
nKeyCode===9){var $elFirstReactionBtn=$.getSingle("a._firstReactionBtn",$elLikeUserListLayer);$elFirstReactionBtn.focus();oEvent.stopPropagation();oEvent.preventDefault()}else if(nKeyCode===13){var nLikeAnswerNo=
$elLikeUserListLayer.data("answerNo")||0;this._closeLikeUserListLayer(nLikeAnswerNo,$elLikeUserListLayer);oEvent.stopPropagation();oEvent.preventDefault()}},
_onClickDocumentBody:function(oEvent){var $elTarget=$(oEvent.target);if($elTarget.hasClass("_likeUserListBtn")||$elTarget.hasClass("_likeUserListLayer"))return;var $elSkipEventArea=$elTarget.closest("._likeUserListBtn",
"._likeUserListLayer");if($elSkipEventArea.existElements())return;this._closeAllLikeUserListLayer()},
_onResizeWindow:function(oEvent){var $elLikeUserListLayers=this._$element["likeUserListLayer"];if(!$elLikeUserListLayers||!$elLikeUserListLayers.existElements())return;for(var i=0,len=$elLikeUserListLayers.length;i<
len;i++){var $elLikeUserListLayer=$($elLikeUserListLayers.get(i));if(!$elLikeUserListLayer.is(":visible"))continue;var nLikeAnswerNo=$elLikeUserListLayer.data("answerNo")||0;this._adjustPositionOfLikeUserListLayer(nLikeAnswerNo,
$elLikeUserListLayer)}},
_onClickLikeUserListButton:function(oEvent){var $elLikeUserListBtn=$(oEvent.currentTarget);var $elAnswerListArea=$elLikeUserListBtn.closest("._answerLikeArea");if(!$elAnswerListArea.existElements())return;
var nLikeAnswerNo=$elAnswerListArea.data("answerNo")||0;var $elLikeUserListLayer=$("#answerLikeUserLayer_"+nLikeAnswerNo);if($elLikeUserListLayer.is(":visible"))this._closeLikeUserListLayer(nLikeAnswerNo,
$elLikeUserListLayer);else{this._closeAllLikeUserListLayer();this._initLikeUserListLayer(nLikeAnswerNo);$elLikeUserListBtn.prop("aria-pressed","true")}},
_closeAllLikeUserListLayer:function(){var $elLikeUserListLayers=this._$element["likeUserListLayer"];if(!$elLikeUserListLayers||!$elLikeUserListLayers.existElements())return;$elLikeUserListLayers.hide();
$elLikeUserListLayers.prop("aria-hidden","true");if(!this._$element["likeUserListBtn"].existElements())return;this._$element["likeUserListBtn"].prop("aria-pressed","false")},
_initLikeUserListLayer:function(nLikeAnswerNo,sReactionType){if(!nLikeAnswerNo){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN);return}if(this._bIsAjaxProcessing)return;$.ajax({url:this._URL.GET_ANSWER_LIKE_COUNT_AJAX,
method:"post",timeout:3E3,data:{docId:this._oOption.docId||0,gdId:this._oOption.gdId||null,answerNo:nLikeAnswerNo},dataType:"json",cache:false,beforeSend:function(){this._bIsAjaxProcessing=true}.bind(this),
complete:function(){this._bIsAjaxProcessing=false}.bind(this),
success:this._onSucceedInAnswerLikeCountAjax.bind(this,nLikeAnswerNo,sReactionType),error:function(jqXHR,sTextStatus,sErrorThrown){alert(sTextStatus==="timeout"?naver.kin.pc.common.MESSAGE.TIMEOUT:naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_onSucceedInAnswerLikeCountAjax:function(nLikeAnswerNo,sReactionType,oResponse){if(nLikeAnswerNo<1){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN);return}if(oResponse&&oResponse.isSuccess){var oAnswerLikeCountStat=
oResponse.stat;if(!oAnswerLikeCountStat){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN);return}if(oAnswerLikeCountStat.kinupCnt<1){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_EMPTY);return}sReactionType=
sReactionType||oResponse.maxReactionType||"like";var $elLikeUserListLayer=$("#answerLikeUserLayer_"+nLikeAnswerNo);var bLayerOpened=$elLikeUserListLayer.is(":visible");if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements()){$elLikeUserListLayer=
$(this._$template["answerLikeUserLayerTpl"]({answerNo:nLikeAnswerNo,reactionType:sReactionType,stat:oAnswerLikeCountStat}));$(document.body).append($elLikeUserListLayer);this._refreshAnswerLikeCountOfLayer($elLikeUserListLayer,
sReactionType,oAnswerLikeCountStat);this._refreshLikeUserListLayer()}else{$.getSingle("em._kinupCnt",$elLikeUserListLayer).text(oAnswerLikeCountStat.kinupCnt);this._refreshAnswerLikeCountOfLayer($elLikeUserListLayer,
sReactionType,oAnswerLikeCountStat);$elLikeUserListLayer.show();$elLikeUserListLayer.prop("aria-hidden","false")}this._adjustPositionOfLikeUserListLayer(nLikeAnswerNo,$elLikeUserListLayer);var $elReactionButtonList=
$elLikeUserListLayer.find("a._reactionTypeBtn");$elReactionButtonList.prop("aria-checked","false");for(var i=0,len=$elReactionButtonList.length;i<len;i++){var $elReactionButton=$($elReactionButtonList.get(i));
if($elReactionButton.data("reactionType")===sReactionType){$elReactionButton.prop("aria-checked","true");if(!bLayerOpened)$($elReactionButtonList.get(0)).focus();else $elReactionButton.focus();break}}this._likeUserListAjax($elLikeUserListLayer,
"init",sReactionType,nLikeAnswerNo,1)}else{var sErrorMsg=oResponse&&oResponse.errorMsg||this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN;alert(sErrorMsg)}},
_refreshAnswerLikeCountOfLayer:function($elLikeUserListLayer,sReactionType,oAnswerLikeCountStat){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements()||!sReactionType||!oAnswerLikeCountStat)return;
var $elAnswerLikeCountArea=$.getSingle("div._answerLikeCountArea",$elLikeUserListLayer);if(!$elAnswerLikeCountArea.existElements())return;$elAnswerLikeCountArea.html(this._$template["answerLikeCountAreaTpl"]({reactionType:sReactionType,
stat:oAnswerLikeCountStat}))},
_refreshLikeUserListLayer:function(){if(this._$element["likeUserListLayer"]&&this._$element["likeUserListLayer"].existElements()){this._$element["likeUserListLayer"].off("click");this._$element["likeUserListLayer"]=
null}this._$element["likeUserListLayer"]=$("div._likeUserListLayer");if(this._$element["likeUserListLayer"].existElements())this._$element["likeUserListLayer"].on("click",this._onClickLikeUserListLayer.bind(this)).on("keydown",
this._onKeydownLikeUserListLayer.bind(this))},
_adjustPositionOfLikeUserListLayer:function(nLikeAnswerNo,$elLikeUserListLayer){if(!nLikeAnswerNo||!$elLikeUserListLayer||!$elLikeUserListLayer.existElements())return;var $elLikeUserListBtn=$.getSingle("#answerLikeArea_"+
nLikeAnswerNo+" a._likeUserListBtn");if(!$elLikeUserListBtn.existElements())return;var oLikeUserListBtnOffset=$elLikeUserListBtn.offset();if(!oLikeUserListBtnOffset)return;var nPaddingOfLikeUserListButton=
20;var nRevisedTopOfLayer=Math.max(Math.ceil(oLikeUserListBtnOffset.top-$elLikeUserListLayer.height()+1),0);var nRevisedLeftOfLayer=Math.ceil(oLikeUserListBtnOffset.left+$elLikeUserListBtn.width()+nPaddingOfLikeUserListButton);
$elLikeUserListLayer.css({"top":nRevisedTopOfLayer+"px","left":nRevisedLeftOfLayer+"px"})},
_likeUserListAjax:function($elLikeUserListLayer,sMode,sReactionType,nLikeAnswerNo,nPage){if(sMode!=="init"&&this._bIsAjaxProcessing){alert(this._MESSAGE.WAIT_FOR_PROCESSING);return}if(sMode==="init"){this._hideAnswerLikeUserList($elLikeUserListLayer);
this._showInnerLayerLoadingIndicator($elLikeUserListLayer)}$.ajax({url:this._URL.GET_ANSWER_LIKE_USER_LIST_AJAX,method:"post",timeout:3E3,data:{reactionType:sReactionType||null,dirId:this._oOption.dirId||
0,docId:this._oOption.docId||0,answerNo:nLikeAnswerNo||0,page:nPage,count:this._COUNT_PER_PAGE},dataType:"json",cache:false,beforeSend:function(){this._bIsAjaxProcessing=true}.bind(this),
complete:function(){this._bIsAjaxProcessing=false}.bind(this),
success:this._onSucceedInAnswerLikeUserListAjax.bind(this,$elLikeUserListLayer,sMode,sReactionType,nLikeAnswerNo,nPage),error:function(jqXHR,sTextStatus,sErrorThrown){alert(sTextStatus==="timeout"?naver.kin.pc.common.MESSAGE.TIMEOUT:
naver.kin.pc.common.MESSAGE.ERROR)}.bind(this)})},
_hideAnswerLikeUserList:function($elLikeUserListLayer){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements())return;var $elLikeUserList=$.getSingle("._likeUserList",$elLikeUserListLayer);$elLikeUserList.hide();
var $elNoLikeUserList=$.getSingle("._noLikeUserList",$elLikeUserListLayer);$elNoLikeUserList.hide()},
_onSucceedInAnswerLikeUserListAjax:function($elLikeUserListLayer,sMode,sReactionType,nLikeAnswerNo,nPage,oResponse){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements()||nLikeAnswerNo<1||nPage<
1){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN);return}if(sMode==="init")this._hideInnerLayerLoadingIndicator($elLikeUserListLayer);if(oResponse&&oResponse.isSuccess){var $elLikeUserList=$.getSingle("._likeUserList",
$elLikeUserListLayer);var $elNoLikeUserList=$.getSingle("._noLikeUserList",$elLikeUserListLayer);if(!$elLikeUserList.existElements()||!$elNoLikeUserList.existElements()){alert(this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN);
return}var aLikeUserList=oResponse.list||[];var nResultCount=aLikeUserList.length;if(nResultCount>0){$elLikeUserList.html(this._$template["answerLikeUserListTpl"]({list:aLikeUserList}));$elLikeUserList.show()}else $elNoLikeUserList.show();
this._refreshLikeUserListPagingBtn($elLikeUserListLayer,sReactionType,nPage,nResultCount)}else{var sErrorMsg=oResponse&&oResponse.errorMsg||this._MESSAGE.FAILED_TO_GET_LIKE_USER_LIST_UNKNOWN;alert(sErrorMsg)}},
_refreshLikeUserListPagingBtn:function($elLikeUserListLayer,sReactionType,nPage,nResultCount){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements())return;var $elPagingBtnArea=$.getSingle("div._pagingBtnArea",
$elLikeUserListLayer);if(!$elPagingBtnArea.existElements())return;var nPrevPage=0;if(nPage>1)nPrevPage=nPage-1;var $elCurrentReactionCount=$.getSingle("span._count._"+sReactionType,$elLikeUserListLayer);
var nCurrentReactionCount=parseInt($elCurrentReactionCount.text(),10)||0;var nNextPage=0;if(nResultCount>=this._COUNT_PER_PAGE&&nCurrentReactionCount>nPage*this._COUNT_PER_PAGE)nNextPage=nPage+1;if(!nPrevPage&&
!nNextPage){$elPagingBtnArea.hide();return}$elPagingBtnArea.html(this._$template["answerLikeUserListPagingTpl"]({reactionType:sReactionType,prevPage:nPrevPage,nextPage:nNextPage}));$elPagingBtnArea.show();
if(this._sButtonClassToBeFocused){this._focusButtonBeSelected(this._sButtonClassToBeFocused,$elPagingBtnArea);this._sButtonClassToBeFocused=null}},
_focusButtonBeSelected:function(className,$elPagingBtnArea){var $elButton=$.getSingle(className,$elPagingBtnArea);if($elButton.existElements())$elButton.focus();else $.getSingle("a",$elPagingBtnArea).focus()},
_showInnerLayerLoadingIndicator:function($elLikeUserListLayer){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements())return;var $elLoadingImage=$.getSingle("div._loadingImage",$elLikeUserListLayer);
$elLoadingImage.show()},
_hideInnerLayerLoadingIndicator:function($elLikeUserListLayer){if(!$elLikeUserListLayer||!$elLikeUserListLayer.existElements())return;var $elLoadingImage=$.getSingle("div._loadingImage",$elLikeUserListLayer);
$elLoadingImage.hide()},
destroy:function(){if(this._$element){this._$element["likeUserListBtn"].off();this._$element["likeUserListLayer"].off()}this._oOption=this._$element=this._$template=this._sReactionModuleClassName=this._sButtonClassToBeFocused=
null;this._bIsAjaxProcessing=null}});
