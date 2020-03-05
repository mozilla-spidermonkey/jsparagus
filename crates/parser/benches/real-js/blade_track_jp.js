(function () {
	function getTrackHost() {
		var track_host="http://d-track.send.microad.jp";
		if(document.location.protocol=="https:"){
			track_host="https://d-track.send.microad.jp";
		}
		return track_host;
	}

	function isItpBrowser(){
		if(navigator.userAgent.match(/Safari/) && navigator.userAgent.match(/Version\/(.\d+)/)){
			return navigator.userAgent.match(/Version\/(.\d+)/)[1] >= 11;
		} else {
			return false;
		}
	}

	function appendCookieSyncIframe(){
		var send_uri = document.location.protocol + '//cache.send.microad.jp';
		var iframe_elem = document.createElement('iframe');
		iframe_elem.width = 1;
		iframe_elem.height = 1;
		iframe_elem.frameborder = 0;
		iframe_elem.setAttribute('style', 'position:absolute; top:-9999px; left: -9999px; border-style: none');
		iframe_elem.src = send_uri + '/js/microad_cookie_sync.html';
		document.body.appendChild(iframe_elem);
	}

	function createCacheBuster(){
		var randomValue = Math.floor(Math.random() * Math.pow(10, 18)).toString(16);
		var date = new Date().getTime().toString(16);
		return randomValue + date;
	}

	function getMicroadEncAidFromUrl(){
		var param_name = 'microad_enc_aid';
		var url_param_array = location.search.substring(1).split('&');
		for (var i = 0; i < url_param_array.length; i++) {
			if (url_param_array[i].split('=')[0] === param_name){
				return url_param_array[i].split('=')[1];
			}
		}
		return '';
	}

	function getMicroadEncAidFromCookie(){
		var res = '';
		var key = 'microad_enc_aid';
		var cookie_array = document.cookie.split( '; ' );
		for (var i = 0; i < cookie_array.length; i++){
			if (cookie_array[i].split('=')[0] === key){
				res = cookie_array[i].split('=')[1];
				setFirstPartyCookie(res);
				break;
			}
		}
		return res;
	}

	function setFirstPartyCookie(microad_enc_aid){
		var period = 365;
		if(microad_enc_aid !== ''){
			document.cookie = 'microad_enc_aid='+microad_enc_aid+'; domain='+document.domain+'; path=/; max-age='+60*60*24*period;
		}
	}

	function isSmartPhone(){
		return /(iPhone|iPod|iPad|Android)/.test(navigator.userAgent);
	}

	function appendMicroadTrackTag(co_account_id, group_id, country_id, version, custom, tag_url) {
		var encode_url=escape(document.referrer);
		var blade_query="co_account_id="+co_account_id+"&group="+group_id+"&country_id="+country_id+"&ver="+version+"&referrer="+encode_url;

		if (custom) {
			blade_query = blade_query + "&custom=" + custom;
		}

		if (tag_url) {
			var encode_tag_url = escape(tag_url);
			blade_query = blade_query + "&url=" + encode_tag_url;
		}

		blade_query = blade_query + "&cbt=" + createCacheBuster();

		var microad_enc_aid =  getMicroadEncAidFromCookie();
		if(microad_enc_aid == ''){
			microad_enc_aid = getMicroadEncAidFromUrl();
		}
		if(microad_enc_aid !== ''){
			blade_query = blade_query + "&enc_aid=" + microad_enc_aid;
		}

		var blade_url = "";
		if(isSmartPhone()){
			blade_query = blade_query + "&cookie=false";
			blade_url=getTrackHost()+"/bl_track_sm.cgi?"+blade_query;
		} else {
			blade_query = blade_query + "&cookie=true";
			blade_url=getTrackHost()+"/bl_track.cgi?"+blade_query;
		}

		var blade_target=new Image();
		blade_target.src=blade_url;
	}

	function microadTrack() {
		var target_params = microad_blade_jp.params;

		for (var i = 0; i < target_params.length; i++) {
			var co_account_id = target_params[i].co_account_id;
			var group_id = target_params[i].group_id;
			var country_id = target_params[i].country_id;
			var version = target_params[i].ver;
			var custom = "";
			if (target_params[i].custom) {
				custom = encodeURIComponent(encodeURIComponent(JSON.stringify(target_params[i].custom)));
			}

			var tag_url = "";
			if(target_params[i].url) {
				tag_url = target_params[i].url;
			}

			var key = co_account_id + '_' + group_id;
			if(key in microad_blade_jp.complete_map) {
				continue;
			}

			appendMicroadTrackTag(co_account_id, group_id, country_id, version, custom, tag_url);

			microad_blade_jp.complete_map[key] = target_params[i];
			break;
		}
	}

	setFirstPartyCookie(getMicroadEncAidFromUrl());
	microadTrack();

	if(!isItpBrowser()){
		appendCookieSyncIframe();
	}

})();
