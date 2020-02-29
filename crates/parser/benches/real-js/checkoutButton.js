function getInnerCheckoutButtonScriptFile() {
	var CHECKOUT_BUTTON_SCRIPT_SRC_PATTERN = /^(https?:\/\/[^\/]*checkout\.naver\.com\/[^?]*)checkoutButton.js/;
	function _findCheckoutButtonSrc() {
		var scriptObjects = document.getElementsByTagName("script");
		if ( scriptObjects == null ) {
			return null;
		}
		for ( var i in scriptObjects ) {
			try {
				var result = CHECKOUT_BUTTON_SCRIPT_SRC_PATTERN.exec(scriptObjects[i].src);
				if ( result != null) {
					return result[1];
				}
			} catch (e) {}
		}
		return null;
	}
	var checkoutButtonSrc = _findCheckoutButtonSrc();
	if (checkoutButtonSrc) {
		document.write('<script type="text/javascript" src="' + checkoutButtonSrc + 'innerCheckoutButton.js?site_preference=normal&' + Math.round(new Date()/3600000) + '" charset="UTF-8"></script>');
	} else {
		alert("HOST 명을 찾을 수 없어 네이버 페이 버튼을 초기화 할 수 없습니다.");
	}
}
getInnerCheckoutButtonScriptFile();