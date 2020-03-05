// var WEBCHAT_BASE_URL = "http://cc.360jie.com.cn/webchat_new/static/";

var WEBCHAT_BASE_URL = document.location.protocol +"//cc.360jie.com.cn/webchat_new/static/";

var WEBCHAT_PC_URL = WEBCHAT_BASE_URL + "google/index.html";

var WEBCHAT_IE_URL = WEBCHAT_BASE_URL + "ie8/index.html";

var WEBCHAT_MOBILE_URL = WEBCHAT_BASE_URL + "moblie/index.html";

var WEBCHAT_WX_URL = WEBCHAT_BASE_URL + "google/index.html";







/**

 * @param imnumber  接入号

 * @param isstatic  固定显示,不可隐藏

 * @param hastab    是否存在右侧页签

 * @param openid    用户openid

 * @param params    自定义参数

 * @param box       自定义是否弹窗参数

 */

function WebchatPop(imnumber, hastab, isstatic, openid, params, box) {

  this.imnumber = imnumber;

  this.isstatic = isstatic;

  this.hastab = hastab || false;

  this.openid = openid || "";

  this.params = params || "";

  this.box = box || "";

}

// 截取网址的域名

function getReferrer () {

  var relUrl =''

  var referrer = ''

  try {

    referrer = window.top.document.referrer

  } catch (e) {

    referrer = ''

  }

  if (referrer === '') {

    relUrl = referrer   

  }else{

    var arrUrl = referrer.split('//')

    var start = arrUrl[1].indexOf('/')

    relUrl = arrUrl[1].substring(0, start)

  }

  return relUrl

}

WebchatPop.prototype.show = function () {

  document.getElementById("adaptation").style.display = "block";

}

WebchatPop.prototype.ready = function (callback) {

  var sourceURL = getReferrer ()

  var win_w = 600;

  var win_h = 450;

  if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {

    var ua = window.navigator.userAgent.toLowerCase();

    if (ua.match(/MicroMessenger/i) == 'micromessenger') {

      var url = WEBCHAT_WX_URL;

    } else {

      url = WEBCHAT_MOBILE_URL;

    }

    url += "?ht=" + this.imnumber;

    if (this.openid && this.openid !== '0') {

      url += "&openid=" + this.openid;

    }

    if (this.params) {

      url += "&params=" + this.params;

    }

    url += "&timestamp=" + new Date().getTime();

    url += "&sourceURL=" + sourceURL;

    window.location.href = url;

  } else {

    if (!this.box) {

      var mydiv = document.createElement('div');
      mydiv.style.position = "fixed";
      mydiv.style.display = "none";

      mydiv.style.width = "600px";

      mydiv.style.height = "450px";

      mydiv.style.overflow = "hidden";

      mydiv.style.margin = "-225px 0 0 -300px";
      mydiv.style.zIndex="1000";
      mydiv.style.top="50%";
      mydiv.style.left="50%";
      mydiv.id = "adaptation";

      var iframe = document.createElement('iframe');

      iframe.src = "";

      iframe.id = "mainiframe";

      iframe.width = "100%";

      iframe.height = "100%";

      iframe.frameBorder = 0;

      mydiv.appendChild(iframe);

      document.body.appendChild(mydiv)

    }

    var url = WEBCHAT_PC_URL;

    if ((navigator.appName == "Microsoft Internet Explorer" && navigator.appVersion.split(";")[1].replace(/[ ]/g, "") == "MSIE8.0") || (navigator.appName == "Microsoft Internet Explorer" && navigator.appVersion.split(";")[1].replace(/[ ]/g, "") == "MSIE9.0") || navigator.userAgent.indexOf("Firefox") > 0) {

      url = WEBCHAT_IE_URL;

    }

    url += "?ht=" + this.imnumber;

    if (this.openid && this.openid !== '0') {

      url += "&openid=" + this.openid;

    }

    if (this.hastab) {

      if (!this.box) {

        document.getElementById("adaptation").style.width = '900px';

        document.getElementById("adaptation").style.height = '600px';

      }

      win_w = 900;

      win_h = 600;

      url += "&hastab=" + this.hastab;

    }

    if (this.params) {

      url += "&params=" + this.params;

    }

    url += "&timestamp=" + new Date().getTime();

    url += "&sourceURL=" + sourceURL;

    if (this.isstatic && this.isstatic == "true") {

      window.location.href = url;

    } else if (this.box) {

      var params_str = "height=" + win_h + ",width=" + win_w + ",toolbar=0,scrollbars=0,location=0,menubar=0,resizable=1,top=50,left=200";

      window.open(url, '_blank', params_str)

    } else {

      document.getElementById("mainiframe").src = url;

    }

  }

  callback(this);

}

