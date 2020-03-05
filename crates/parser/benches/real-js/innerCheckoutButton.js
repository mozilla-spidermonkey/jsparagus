//////////////////////////////////////////////////////////////////////////////// 
// 
// NHN Business Platform CORPORATION
// Copyright 2009-2013 NHN Business Platform CORPORATION 
// All Rights Reserved. 
// 
// 이 문서는 NHN Business Platform Corp.㈜의 지적 자산이므로 NHN Business Platform Corp.(주)의 승인 없이 이 문서를 다른 용도로 임의 
// 변경하여 사용할 수 없습니다. 
// 
// 파일명 : checkoutButton.js 
// 작성일 : 2010.01.13 
// 
// 최종 수정일: 2013.02.27
// 
// Version : 1.3
// 
////////////////////////////////////////////////////////////////////////////////

/**
 * @author Gookeun Lim / passion.lim@nhn.com 
 * modify taeheung.ha@nhn.com 
 * modify hyundo.ryu@nhn.com
 */

if (typeof nhn == 'undefined') nhn = {};

nhn.CheckoutButton = (function(){
	// 변수 정의
	var CHECKOUT_BUTTON_STYLE_ID="NAVER_CHECKOUT_STYLE";
	var CHECKOUT_HOST_PATTERN = /^https?:\/\/[^\/]*checkout\.naver\.com/;
	var CHECKOUT_BUTTON_SRC_PATTERN = /^(https?:\/\/[^\/]*checkout\.naver\.com)\/[^?]*\/innerCheckoutButton.js/;
	var CHECKOUT_URL = "checkout.naver.com";
	var LAYOUT_URL = "img.pay.naver.net";
	var FORCE_DISABLE_MESSAGE_KEY = "FORCE_DISABLE";
	
	var BUTTON_TYPE_PARAM = {
		"A_1_1" : {"BUTTON_CLASS" : "npay_type_A_1",   "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"A_1_2" : {"BUTTON_CLASS" : "npay_type_A_2",   "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"B_1_1" : {"BUTTON_CLASS" : "npay_type_B_1",   "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"B_1_2" : {"BUTTON_CLASS" : "npay_type_B_2",   "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		                                                                                                                     
		"C_1_1" : {"BUTTON_CLASS" : "npay_type_C_1_1", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"C_1_2" : {"BUTTON_CLASS" : "npay_type_C_1_2", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"C_2_1" : {"BUTTON_CLASS" : "npay_type_C_2_1", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"C_2_2" : {"BUTTON_CLASS" : "npay_type_C_2_2", "NPAY_BUTTON_BOX_CLASS" : ""											 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"C_3_1" : {"BUTTON_CLASS" : "npay_type_C_3_1", "NPAY_BUTTON_BOX_CLASS" : "npay_bg_dgray" 							 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"C_3_2" : {"BUTTON_CLASS" : "npay_type_C_3_2", "NPAY_BUTTON_BOX_CLASS" : "npay_bg_dgray" 							 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		                                                                                                                     
		"D_1_1" : {"BUTTON_CLASS" : "npay_type_D_1_1", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"D_1_2" : {"BUTTON_CLASS" : "npay_type_D_1_2", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"D_2_1" : {"BUTTON_CLASS" : "npay_type_D_2_1", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"D_2_2" : {"BUTTON_CLASS" : "npay_type_D_2_2", "NPAY_BUTTON_BOX_CLASS" : "" 										 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"D_3_1" : {"BUTTON_CLASS" : "npay_type_D_3_1", "NPAY_BUTTON_BOX_CLASS" : "npay_bg_dgray" 							 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"D_3_2" : {"BUTTON_CLASS" : "npay_type_D_3_2", "NPAY_BUTTON_BOX_CLASS" : "npay_bg_dgray" 							 , "NPAY_BUY_LINK_STYLE":"", "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		                                                                                                                     
		"E_1_1" : {"BUTTON_CLASS" : "npay_type_E_1_1", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical"							 , "NPAY_BUY_LINK_STYLE":"",                                         "NPAY_WISH_LINK_STYLE":"", "NPAY_TALK_LINK_STYLE":""},
		"E_1_2" : {"BUTTON_CLASS" : "npay_type_E_1_2", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical npay_vertical2" 			 , "NPAY_BUY_LINK_STYLE":"",                                         "NPAY_WISH_LINK_STYLE":"background-position:50% -124px !important", "NPAY_TALK_LINK_STYLE":""},
		"E_2_1" : {"BUTTON_CLASS" : "npay_type_E_2_1", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical"							 , "NPAY_BUY_LINK_STYLE":"background-position:50% -31px !important", "NPAY_WISH_LINK_STYLE":"",                                          "NPAY_TALK_LINK_STYLE":"background-position:50% -31px !important"},
		"E_2_2" : {"BUTTON_CLASS" : "npay_type_E_2_2", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical npay_vertical2" 			 , "NPAY_BUY_LINK_STYLE":"background-position:50% -31px !important", "NPAY_WISH_LINK_STYLE":"background-position:50% -124px !important", "NPAY_TALK_LINK_STYLE":"background-position:50% -31px !important"},
		"E_3_1" : {"BUTTON_CLASS" : "npay_type_E_3_1", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical npay_bg_dgray"	 			 , "NPAY_BUY_LINK_STYLE":"background-position:50% -64px !important", "NPAY_WISH_LINK_STYLE":"",                                          "NPAY_TALK_LINK_STYLE":"background-position:50% -62px !important"},
		"E_3_2" : {"BUTTON_CLASS" : "npay_type_E_3_2", "NPAY_BUTTON_BOX_CLASS" : "npay_vertical npay_vertical2 npay_bg_dgray", "NPAY_BUY_LINK_STYLE":"background-position:50% -64px !important", "NPAY_WISH_LINK_STYLE":"background-position:50% -155px !important", "NPAY_TALK_LINK_STYLE":"background-position:50% -62px !important"},
		
		"F_1_2" : {"BUTTON_CLASS" : "npay_type_F npay_store", 	   "NPAY_BUTTON_BOX_CLASS" : ""          },
		"G_1_1" : {"BUTTON_CLASS" : "npay_type_G",     "NPAY_BUTTON_BOX_CLASS" : "npay_horizontal"},
		
		"" : ""
	};
	
	function _prepareDIV(optionParams) {
		BUTTON_DIV = ''
			+'<div id="${BUTTON_ID}" class="npay_storebtn_bx ${BUTTON_CLASS}">'
			+ _prepareInnerDIV(optionParams)
			+'</div>';
		return BUTTON_DIV;
	}
	
	function _prepareInnerDIV(optionParams) {
		BUTTON_INNER_DIV = ''
			+'	<div id="NPAY_BUTTON_BOX_ID" class="npay_button_box ${NPAY_BUTTON_BOX_CLASS}">'
			+'		<div class="npay_button">'
			+'			<div class="npay_text"><span class="npay_blind">NAVER 네이버 ID로 간편구매 네이버페이</span></div>'
			+'				${NPAY_BUTTON}'
			+'		</div>'
			+'		<div id="NPAY_EVENT_ID" class="npay_event">'
			+'			<a id="NPAY_PROMOTION_PREV_ID'+optionParams['BUTTON_ID']+'" href="#" class="npay_more npay_more_prev"><span class="npay_blind">이전</span></a>'
			+'			<p id="NPAY_PROMOTION_ID'+optionParams['BUTTON_ID']+'" class="npay_event_text"><strong class="event_title">이벤트</strong><a href="http://event2.pay.naver.com/event/benefit/list" class="event_link" target="_blank">네이버페이</a></p>'
			+'			<a id="NPAY_PROMOTION_NEXT_ID'+optionParams['BUTTON_ID']+'" href="#" class="npay_more npay_more_next"><span class="npay_blind">다음</span></a>'
			+'		</div>'
			+'	</div>'
		return BUTTON_INNER_DIV;
	}

	function _prepareButtonHTML(optionParams) {
		ENABLE = optionParams['ENABLE'];
		var showTalkButton = optionParams['talk'] == 'Y' && ENABLE == 'Y';
		buttonHTML = '';
		
		if(optionParams["BUTTON_TYPE"] == 'E_1_2' || optionParams["BUTTON_TYPE"] == 'E_2_2' || optionParams["BUTTON_TYPE"] == 'E_3_2') {
				buttonHTML = buttonHTML
				+'<table class="npay_btn_list" cellspacing="0" cellpadding="0">'
				+'<tr>'
				+'	<td class="npay_btn_item">'
				+'		<a ID="NPAY_BUY_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_BUY_LINK_URL}" class="npay_btn_link npay_btn_pay ${NPAY_BUY_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_BUY_LINK_STYLE}" title="새창"><span class="npay_blind">네이버페이 구매하기</span></a>'
				+'	</td>'
				+'</tr>'
				+'</table>'
				+'<table class="npay_btn_list npay_btn_margin" cellspacing="0" cellpadding="0">'
				+'<tr>'
				+'	<td class="npay_btn_item">'
				+'		<a ID="NPAY_WISH_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_WISH_LINK_URL}" class="npay_btn_link npay_btn_zzim2 ${NPAY_WISH_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_WISH_LINK_STYLE}" title="새창"><span class="npay_blind">찜하기</span></a>'
				+'	</td>';
			if(showTalkButton) {
				buttonHTML = buttonHTML
				+'	<td class="npay_btn_item btn_width">'
				+'		<a ID="NPAY_TALK_LINK_ID'+optionParams['BUTTON_ID']+'" href="#" class="npay_btn_link npay_btn_talk ${NPAY_TALK_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_TALK_LINK_STYLE}" title="새창"><span class="npay_blind">톡톡</span></a>'
				+'	</td>';
			}	
				buttonHTML = buttonHTML
				+'</tr>'
				+'</table>';
		} else if(optionParams["BUTTON_TYPE"] == 'F_1_2') {
				buttonHTML = buttonHTML
				+'<table class="npay_btn_list npay_btn_gray" cellspacing="0" cellpadding="0">'
				+'<tr>'
	            +' <td class="npay_btn_item">'
	            +'     <a ID="NPAY_BUY_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_BUY_LINK_URL}" class="npay_btn_link npay_btn_pay ${NPAY_BUY_LINK_CLASS}" style="box-sizing:content-box;" title="새창"><span class="npay_blind">구매하기</span></a>'
	            +' </td>'
	            +' <td class="npay_btn_item btn_width">'
	            +'     <a ID="NPAY_CART_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_CART_LINK_URL}" class="npay_btn_link npay_btn_cart ${NPAY_CART_LINK_CLASS}" style="box-sizing:content-box;" title="새창"><span class="npay_blind">장바구니</span></a>'
	            +' </td>';
			if(showTalkButton) {
				buttonHTML = buttonHTML
	            +' <td class="npay_btn_item btn_width">'
	            +'     <a ID="NPAY_TALK_LINK_ID'+optionParams['BUTTON_ID']+'" href="#" class="npay_btn_link npay_btn_talk2 ${NPAY_TALK_LINK_CLASS}" style="box-sizing:content-box;" title="새창"><span class="npay_blind">톡톡</span></a>'
	            +' </td>';
	         }
	         	buttonHTML = buttonHTML
	         	+'</tr>'
	            +'</table>';
		} else if(optionParams["BUTTON_TYPE"] == 'G_1_1') { 
				buttonHTML = buttonHTML
		        +'<table  class="npay_btn_list" cellspacing="0" cellpadding="0">'
		        +'<tr>'
		        +'    <td class="npay_btn_item">'
	            +'        <a ID="NPAY_BUY_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_BUY_LINK_URL}" class="npay_btn_link npay_btn_pay ${NPAY_BUY_LINK_CLASS}" style="box-sizing:content-box;" title="새창"><span class="npay_blind">구매하기</span></a>'
	            +'    </td>'
	            +'</tr>'
	            +'</table >';
		} else {
				buttonHTML = buttonHTML
				+'<table class="npay_btn_list" cellspacing="0" cellpadding="0">'
				+'<tr>'
			    +'    <td class="npay_btn_item">'
			    +'        <a ID="NPAY_BUY_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_BUY_LINK_URL}" class="npay_btn_link npay_btn_pay ${NPAY_BUY_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_BUY_LINK_STYLE}" title="새창"><span class="npay_blind">네이버페이 구매하기</span></a>'
			    +'    </td>';
			if(optionParams["COUNT"] == 2){
				buttonHTML = buttonHTML
			    +'    <td class="npay_btn_item btn_width">'
			    +'        <a ID="NPAY_WISH_LINK_ID'+optionParams['BUTTON_ID']+'" href="${NPAY_WISH_LINK_URL}" class="npay_btn_link npay_btn_zzim ${NPAY_WISH_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_WISH_LINK_STYLE}" title="새창"><span class="npay_blind">찜하기</span></a>'
			    +'    </td>';
			}
			
			if(showTalkButton) {
				buttonHTML = buttonHTML
			    +'    <td class="npay_btn_item btn_width">'
			    +'        <a ID="NPAY_TALK_LINK_ID'+optionParams['BUTTON_ID']+'" href="#" class="npay_btn_link npay_btn_talk ${NPAY_TALK_LINK_CLASS}" style="box-sizing:content-box; ${NPAY_TALK_LINK_STYLE}" title="새창"><span class="npay_blind">톡톡</span></a>'
			    +'    </td>';
			}    
				buttonHTML = buttonHTML
				+'</tr>'
			    +'</table>';
		}
		
		return buttonHTML;
	}
	
	function _openTalk(takDummyArg, talkAccountId) {
		if(typeof(talkAccountId)=='undefined') {
			alert('잠시 후 다시 시도하세요.');
			return;
		}
		talkUrl = _getTalkDomain()+talkAccountId+'?frm=ppay&ref='+ encodeURIComponent(location.href)+'#nafullscreen';
		window.open(talkUrl,'talkWindow','width=471, height=640, resizable=yes, toolbar=no, menubar=no');
	}
	
	function _getTalkDomain(){
		if(GLOBALS.HOST.indexOf('dev-')>=0 || GLOBALS.HOST.indexOf('alpha-')>=0) {
			return 'https://dev-talk.naver.com/';
		} else if(GLOBALS.HOST.indexOf('beta-')>=0) {
			return 'https://beta-talk.naver.com/';
		} else {
			return 'https://talk.naver.com/';
		} 
	} 
	
	var Buttons = {};
	var CheckoutButton = {};
	
	CheckoutButton.putAllTo = function(dest, src) {
		for (var key in src ) {
			dest[key] = src[key];
		}
		return dest;
	};
	
	CheckoutButton.replace = function(buttonHTML, orgText, repText) {
		var buttonHTML = buttonHTML.replace(orgText, repText);
		return buttonHTML;
	};
	
	CheckoutButton.prepareInitialParam = function(optionParams) {
		CheckoutButton.putAllTo(optionParams, GLOBALS);
		
		optionParams["BUTTON_ID"] = "NC_ID_"+new Date().getTime() + parseInt(Math.random() * 1000);
		optionParams["COUNT"] = optionParams["COUNT"] == 3 ? 2 : optionParams["COUNT"];
		optionParams["BUTTON_TYPE"] = optionParams["TYPE"]+"_"+optionParams["COLOR"]+"_"+optionParams["COUNT"];
		
		buttonTypeParam = BUTTON_TYPE_PARAM[optionParams["BUTTON_TYPE"]];
		if(buttonTypeParam == null || typeof(buttonTypeParam)=='undefined') {
			optionParams["TYPE"] = 'A';
			optionParams["COLOR"] = 1;
			optionParams["COUNT"] = 1;
			optionParams["ENABLE"] = 'N';
			optionParams["BUTTON_TYPE"] = optionParams["TYPE"]+"_"+optionParams["COLOR"]+"_"+optionParams["COUNT"];
		}
		
		if(optionParams["BUY_BUTTON_LINK_URL"] == '' || optionParams["BUY_BUTTON_LINK_URL"] == null) {
			optionParams["BUY_BUTTON_LINK_URL"] = '#';
		}
		
		if(optionParams["WISHLIST_BUTTON_LINK_URL"] == '' || optionParams["WISHLIST_BUTTON_LINK_URL"] == null) {
			optionParams["WISHLIST_BUTTON_LINK_URL"] = '#';
		}
		
		if(optionParams["CART_BUTTON_LINK_URL"] == '' || optionParams["CART_BUTTON_LINK_URL"] == null) {
			optionParams["CART_BUTTON_LINK_URL"] = '#';
		}
	};
	
	CheckoutButton.prepareLayoutHTML = function(optionParams) {
		buttonType = optionParams["BUTTON_TYPE"];
		buttonParam = BUTTON_TYPE_PARAM[buttonType];
		
		layoutHTML = _prepareDIV(optionParams);	
		layoutHTML = CheckoutButton.replace(layoutHTML, '${BUTTON_ID}', optionParams["BUTTON_ID"]);
		layoutHTML = CheckoutButton.replace(layoutHTML, '${BUTTON_CLASS}',	buttonParam.BUTTON_CLASS);
		layoutHTML = CheckoutButton.replace(layoutHTML, '${NPAY_BUTTON_BOX_CLASS}', buttonParam.NPAY_BUTTON_BOX_CLASS);
		optionParams["layoutHTML"] = layoutHTML;
	};
	
	CheckoutButton.prepareButtonHTML = function(optionParams) {
		buttonHTML = _prepareButtonHTML(optionParams);
		
		buttonType = optionParams["BUTTON_TYPE"];
		buttonParam = BUTTON_TYPE_PARAM[buttonType];
		
		buttonParam['NPAY_BUY_LINK_CLASS'] = 'btn_green';
		if( buttonType == 'C_1_1' || buttonType == 'C_1_2' || buttonType == 'D_1_1' || buttonType == 'D_1_2' || buttonType == 'E_1_1' || buttonType == 'E_1_2' ) {
			buttonParam.NPAY_BUY_LINK_CLASS = '';
		}
		
		if(optionParams["ENABLE"] == 'Y') {
			buttonParam.NPAY_WISH_LINK_CLASS = '';
			buttonParam.NPAY_TALK_LINK_CLASS = '';
			buttonParam.NPAY_CART_LINK_CLASS = '';
		} else {
			buttonParam.NPAY_BUY_LINK_CLASS = 'btn_gray';
			buttonParam.NPAY_WISH_LINK_CLASS = 'btn_gray';
			buttonParam.NPAY_TALK_LINK_CLASS = 'btn_gray';
			buttonParam.NPAY_CART_LINK_CLASS = 'btn_gray';
		}
		
		buyButtonStyle="";
		talkButtonStyle="";
		wishButtonStyle="";
		if(optionParams["ENABLE"] == 'N' && (buttonType == 'E_1_2' || buttonType == 'E_2_1' || buttonType == 'E_2_2' || buttonType == 'E_3_1' || buttonType == 'E_3_2')) {
			buyButtonStyle = buttonParam.NPAY_BUY_LINK_STYLE;
			wishButtonStyle= buttonParam.NPAY_WISH_LINK_STYLE;
			talkButtonStyle = buttonParam.NPAY_TALK_LINK_STYLE;
		}
		
		// add css class into template
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_BUY_LINK_CLASS}', buttonParam.NPAY_BUY_LINK_CLASS);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_WISH_LINK_CLASS}', buttonParam.NPAY_WISH_LINK_CLASS);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_TALK_LINK_CLASS}', buttonParam.NPAY_TALK_LINK_CLASS);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_CART_LINK_CLASS}', buttonParam.NPAY_TALK_LINK_CLASS);
		
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_BUY_LINK_STYLE}', buyButtonStyle);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_WISH_LINK_STYLE}', wishButtonStyle);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_TALK_LINK_STYLE}', talkButtonStyle);
		
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_BUY_LINK_URL}', optionParams["BUY_BUTTON_LINK_URL"]);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_WISH_LINK_URL}', optionParams["WISHLIST_BUTTON_LINK_URL"]);
		buttonHTML = CheckoutButton.replace(buttonHTML, '${NPAY_CART_LINK_URL}', optionParams["CART_BUTTON_LINK_URL"]);
		
		optionParams['buttonHTML'] = buttonHTML;
	};
	
	CheckoutButton.apply = function(optionParams) {
		CheckoutButton.prepareInitialParam(optionParams);
		CheckoutButton.prepareLayoutHTML(optionParams);
		CheckoutButton.prepareButtonHTML(optionParams);
		
		var htmlTemplate = optionParams['layoutHTML'];
		htmlTemplate = htmlTemplate.replace('${NPAY_BUTTON}', optionParams['buttonHTML']);
		
		if ( optionParams["EMBED_ID"] != null ) {
			var embededElement = document.getElementById(optionParams["EMBED_ID"]);
			if ( embededElement == null) {
				alert("버튼을 넣을 수 있는 Element 를 찾을 수 없습니다. EMBED_ID="+optionParams["EMBED_ID"]);
				return false;
			}
			var tmpPayChild = document.createElement("DIV");
			tmpPayChild.innerHTML = htmlTemplate;
			
			if(embededElement.childNodes[1] && embededElement.childNodes[1].tagName == 'DIV'){
				embededElement.removeChild(embededElement.childNodes[1]);
			}
			if(embededElement.childNodes[2] && embededElement.childNodes[2].tagName == 'DIV'){
				embededElement.removeChild(embededElement.childNodes[2]);
			}
			embededElement.appendChild(tmpPayChild.childNodes[0]);
		} else {
			document.write(htmlTemplate);
		}
		
		var buttonDiv = document.getElementById(optionParams["BUTTON_ID"]);
		var newChild = document.createElement(buttonDiv.tagName);
        newChild.id = buttonDiv.id;
        newChild.className = buttonDiv.className;
        newChild.innerHTML = buttonDiv.innerHTML;
        buttonDiv.parentNode.replaceChild(newChild, buttonDiv);
        
		if (optionParams["BUTTON_KEY"] == "common") {
			return optionParams["BUTTON_ID"];
		}
		
		CheckoutButton.applyAllButtonHandler(optionParams);
		
		var jsonRequestUrl = GLOBALS.HOST + "/button/info";
		jsonRequestUrl += "?callback=nhn.CheckoutButton.jsonp.callback";
		jsonRequestUrl += "&buttonKey=" + optionParams["BUTTON_KEY"];
		jsonRequestUrl += "&buttonTypeCode=" + optionParams["BUTTON_TYPE"]+'_'+optionParams["ENABLE"];
		jsonRequestUrl += "&site_preference=normal";
		
		CheckoutButton.jsonp(jsonRequestUrl);
		Buttons[optionParams["BUTTON_ID"]] = {"ID" : optionParams["BUTTON_ID"], "KEY" : optionParams["BUTTON_KEY"], "TYPE" : optionParams["BUTTON_TYPE"], "SET" : false, "PARAM" : optionParams, "":""};
		
		return optionParams["BUTTON_ID"];
	};
	
	// 이벤트 버튼 이벤트 적용
	CheckoutButton.applyAllButtonHandler = function(optionParams, forceDisableMessage) {

		if (!forceDisableMessage) {
			if(document.getElementById("NPAY_BUY_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_BUY_LINK_ID"+optionParams['BUTTON_ID']), optionParams["BUY_BUTTON_HANDLER"], optionParams["BUY_BUTTON_HANDLER_ARGS"]);
			}
			if(document.getElementById("NPAY_WISH_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_WISH_LINK_ID"+optionParams['BUTTON_ID']), optionParams["WISHLIST_BUTTON_HANDLER"], optionParams["WISHLIST_BUTTON_HANDLER_ARGS"]);
			}
			if(document.getElementById("NPAY_CART_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_CART_LINK_ID"+optionParams['BUTTON_ID']), optionParams["CART_BUTTON_HANDLER"], optionParams["CART_BUTTON_HANDLER_ARGS"]);
			}
			if(document.getElementById("NPAY_TALK_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_TALK_LINK_ID"+optionParams['BUTTON_ID']), _openTalk, [optionParams['talkAccountId']]);
			}		
		} else {
			if(document.getElementById("NPAY_BUY_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_BUY_LINK_ID"+optionParams['BUTTON_ID']), function(){alert(forceDisableMessage); return false;});
			}
			if(document.getElementById("NPAY_WISH_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_WISH_LINK_ID"+optionParams['BUTTON_ID']), function(){alert(forceDisableMessage); return false;});
			}
			if(document.getElementById("NPAY_CART_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_CART_LINK_ID"+optionParams['BUTTON_ID']), function(){alert(forceDisableMessage); return false;});
			}
			if(document.getElementById("NPAY_TALK_LINK_ID"+optionParams['BUTTON_ID'])) {
				CheckoutButton.applyButtonHandler(document.getElementById("NPAY_TALK_LINK_ID"+optionParams['BUTTON_ID']), function(){alert(forceDisableMessage); return false;});
			}	
		}
	};
	
	CheckoutButton.applyButtonHandler = function(element, handler, argsParam) {
		var args = argsParam!=null?argsParam:new Array();
		var shifted = false;
		if ( handler != null && element != null ) {
			element.onclick = function(){
				if ( !shifted ) {
					args.unshift(this.href);
					shifted = true;
				}
				var result = handler.apply(this, args); 
				return result!=null?result:false; 
			};
		}
	};
	
	CheckoutButton.jsonp = function(url) {
		var head = document.getElementsByTagName("head").item(0);
		  if(head == null || typeof(head) == "undefined") {
			  head = document.body;
		   }
	      var script = document.createElement("script");
	      script.setAttribute("type", "text/javascript");
	      script.setAttribute("charset", "utf-8");
	      script.setAttribute("src", url);
	      head.appendChild(script);
	};
	
	CheckoutButton.jsonp.callback = function(json) {
		if (typeof json != "object" || !("KEY" in json) || !("TYPE" in json) || !("STATE" in json)) {
			return;
		}
		
		// 버튼이 여러개 있을 경우를 대비해서
		var applyButtons =  new Array();
		var applyButtonsIndex = 0;
		for (var id in Buttons ) {
			if ((Buttons[id].KEY == json.KEY || FORCE_DISABLE_MESSAGE_KEY in json) && Buttons[id].SET == false) {
				Buttons[id].SET = true;
				applyButtons[applyButtonsIndex] = Buttons[id].ID;
				applyButtonsIndex++;
			} 
		}
		
		for(var i = 0 ; i < applyButtons.length; i++) {
			var id = applyButtons[i];
			
			var buttonDiv = document.getElementById(id);
			if (!buttonDiv) {
				continue;
			}
			
			var changed = false;
			if (!json.STATE) {
				Buttons[id].PARAM["ENABLE"] = "N";
				changed = true;
			}
			
			if (Buttons[id].PARAM["COUNT"] == 3) {
				Buttons[id].PARAM["COUNT"] = 2;
				changed = true;
			}
			
			if (FORCE_DISABLE_MESSAGE_KEY in json) {
				Buttons[id].PARAM["ENABLE"] = "N";
				changed = true;
			}
			
			if(json.talk == 'Y') {
				changed = true;
				Buttons[id].PARAM["talk"] = 'Y';
				Buttons[id].PARAM["talkAccountId"] = json.talkAccountId;
			}
			
			if (changed) {
				optionParams = Buttons[id].PARAM;
				CheckoutButton.prepareButtonHTML(optionParams);
				var htmlTemplate = _prepareInnerDIV(optionParams);
				htmlTemplate = htmlTemplate.replace('${NPAY_BUTTON_BOX_CLASS}', buttonParam.NPAY_BUTTON_BOX_CLASS);
				htmlTemplate = htmlTemplate.replace('${NPAY_BUTTON}', optionParams['buttonHTML']);
				
				var newChild = document.createElement(buttonDiv.tagName);
		        newChild.id = buttonDiv.id;
		        newChild.className = buttonDiv.className;
		        newChild.innerHTML = htmlTemplate;
		        buttonDiv.parentNode.replaceChild(newChild, buttonDiv);
				CheckoutButton.applyAllButtonHandler(optionParams, json[FORCE_DISABLE_MESSAGE_KEY]);
			}
			
			if ("PRMT" in json) {
				CheckoutButton.applyPromotion(id, json.PRMT, Buttons[id].PARAM);
			}
		}
	};
	
	var PromotionCheck = {};
	CheckoutButton.applyPromotion = function(buttonIdParam, promotionsParam, buttonParam) {
		if (PromotionCheck.buttonId || typeof promotionsParam != "object") {
			return;
		}
		
		PromotionCheck[buttonId] = true;
		
		if (promotionsParam.length == 0) {
			return;
		}
		var buttonId = buttonIdParam;
		var promotions = promotionsParam;
		var index = 0;
		var oRolling;
		
		var elPromotionPrevBtn = document.getElementById("NPAY_PROMOTION_PREV_ID"+buttonIdParam);
		var elPromotionNextBtn = document.getElementById("NPAY_PROMOTION_NEXT_ID"+buttonIdParam);
		var elPromotionArea = document.getElementById("NPAY_PROMOTION_ID"+buttonIdParam);
		var elEventArea = document.getElementById("NPAY_EVENT_ID");
		
		if (!elPromotionArea) {
			return ;
		}

		function changePromotion(index) {
			var promotion = promotions[index];
			if(elPromotionArea.childNodes[0]) {
				var title = '';
				var classNM = 'npay_event_text';
				if("title" in promotion && promotion.title != null && promotion.title != '') {
					title = promotion.title;
				}

				if("pointplus" in promotion) {
					classNM = "npay_event_text npay_pointplus";
				}

				elPromotionArea.className = classNM;
				elPromotionArea.childNodes[0].innerHTML = title;
			}

			var childList = elPromotionArea.getElementsByTagName("A");
			if(childList[0]) {
				var newChild = document.createElement(childList[0].tagName);
				newChild.className = childList[0].className;
				newChild.href = promotion.link;
				newChild.target = "_blank";
				newChild.title = "새창";

				var sloganContent = promotion.sloganContent;
				if (!"pointplus" in promotion && buttonParam["BUTTON_TYPE"].indexOf('E_') > -1 && promotion.sloganContent.length > 6) {
					sloganContent = promotion.sloganContent.substr(0, 6) + '..';
				}

				newChild.innerHTML = sloganContent;
				childList[0].parentNode.replaceChild(newChild, childList[0]);
			}
		}


		function rolling(next) {
			index += next;
			if (index < 0) {
				index = promotions.length - 1;
			} else if (index >= promotions.length) {
				index = 0;
			}
			changePromotion(index);
		}
		
		function rollingNext() {
			if (promotions.length > 1) {
				rolling(1);
			}
		}

		function rollingPrev() {
			if (promotions.length > 1) {
				rolling(-1);
			}
		}
		
		function onLoad() {
			changePromotion(index);
			if (promotions.length > 1) {
				if (elPromotionPrevBtn && elPromotionNextBtn) {
					CheckoutButton.applyButtonHandler(elPromotionPrevBtn, rollingPrev);
					CheckoutButton.applyButtonHandler(elPromotionNextBtn, rollingNext);
				}
				oRolling = setInterval(rollingNext, 5000);
				overOutPromotion();
				focusFocusOutPromotion();
			}
		}
		
		function clearRolling() {
			clearInterval(oRolling)
		}
		
		function setRolling() {
			oRolling=setInterval(rollingNext, 5000);
		}
		
		function overOutPromotion(){
			elEventArea.addEventListener("mouseover", clearRolling);
			elEventArea.addEventListener("mouseout", setRolling);
		}
		
		function focusFocusOutPromotion(){
			elPromotionPrevBtn.addEventListener("focusin", clearRolling);
			elPromotionPrevBtn.addEventListener("focusout", setRolling);
			elPromotionNextBtn.addEventListener("focusin", clearRolling);
			elPromotionNextBtn.addEventListener("focusout", setRolling);
			
			elPromotionArea.addEventListener("focusin", clearRolling);
			elPromotionArea.addEventListener("focusout", setRolling);
		}
		
		onLoad();
	};
	
	//--- GLOBALS 변수 세팅 및 CSS 적용--------------------------------------------------------//
	// GLOBALS 변수 선언 되어 있으면 실행 안함
	if (typeof GLOBALS != 'undefined' ) {
		return;
	}

	// HOST 세팅
	function _findCheckoutHost() {
		var scriptObjects = document.getElementsByTagName("script");
		if ( scriptObjects == null ) {
			return null;
		}
		for ( var i in scriptObjects ) {
			try {
				var result = CHECKOUT_BUTTON_SRC_PATTERN.exec(scriptObjects[i].src);
				if ( result != null) {
					return result[1];
				}
			} catch(e) {}
		}
		return null;
	}
	
	var GLOBALS = {
		HOST:_findCheckoutHost(this)
	};
	
	// HOST 검증및 LAYOUTHOST 세팅 
	function _validateGlobalVariable() {
		if ( GLOBALS.HOST == null ) {
			alert("HOST 명을 찾을 수 없어 네이버 페이 버튼을 초기화 할 수 없습니다.");
			return false;
		} else if ( !CHECKOUT_HOST_PATTERN.test(GLOBALS.HOST) ) {
			alert("HOST 명이 유효하지 않습니다.");
			return false;
		}
		GLOBALS.LAYOUTHOST = GLOBALS.HOST.replace(CHECKOUT_URL, LAYOUT_URL);
		return true;
	}
	

	if (!_validateGlobalVariable()) {
		return;
	}
	
	// CSS 적용
	function _embedStyleSheet() {	
		document.write("<link id='"+CHECKOUT_BUTTON_STYLE_ID+"' type='text/css' rel='stylesheet' href='"+GLOBALS.LAYOUTHOST+"/static/css/button/button2.css?" + Math.round(+new Date()/3600000) + "' />");
	}
	
	_embedStyleSheet();

	return CheckoutButton;
})();