var csdn = csdn || function () { };

csdn.doing = false;
csdn.ck_un = "UserName";
csdn.ck_user = "UserInfo";

csdn.val = function (e) {
    return $.trim(((typeof e == "string") ? $("#" + e) : e).val());
};
csdn.val2 = function (e) {
    return encodeURIComponent(csdn.val(e));
};
csdn.hasVal = function (e) {
    var v = csdn.val(e);
    return (v != "" && v != "请选择");
};

/*判断URL中是否包含字符串s*/
csdn.urlHas = function (s) {
    return window.location.href.toLowerCase().indexOf(s.toLowerCase()) > 0;
};
/*获取一个指定长度随机数*/
csdn.random = function (len) {
    if (!len) len = 5;
    var r = Math.random().toString();
    return r.substr(r.length - len);
};

csdn.parent = (parent || top || window);
csdn.toJSON = function (data) {
    if (typeof data == "string") data = eval("(" + data + ")");
    return data;
};
/*q:参数名; url:可选*/
csdn.getQuery = function (q, url) {
    if (!url) url = window.location + '';
    else url += '';
    var reg = new RegExp("[?&](" + q + ")=([^&]+)", "i");
    var re = reg.exec(url);
    if (re) return decodeURIComponent(re[2].replace(/[+]/g,' '));
    else return "";
};
csdn.format = function (s, pars) {
    if (!s) return "";
    if (pars == null) return s;

    var i = 0, j = 1;
    while (j < arguments.length) {
        var arg = arguments[j];
        if (!arg) arg = '';
        if (csdn.isArray(arg)) {
            for (var k = 0; k < arg.length; k++) {
                s = s.replace(new RegExp("\\\{" + (i++) + "\\\}", "g"), arg[k]);
            }
        } else {
            s = s.replace(new RegExp("\\\{" + (i++) + "\\\}", "g"), arg.toString());
        }
        j++;
    }
    return s;
};
csdn.isArray = function (val) {
    return !!(val &&
        typeof val == "object" &&
        typeof val.length == 'number' &&
        typeof val.splice == 'function' &&
        !(val.propertyIsEnumerable('length'))
        );
};
csdn.htmlEncode = function (s) {
    if (!s) return "";

    s = s.replace(/[&]/g, "&amp;");
    s = s.replace(/[<]/g, "&lt;");
    s = s.replace(/[>]/g, "&gt;");
    return s;
};
csdn.urls = window.location.href.toLowerCase().substr(7).split('/');

/*判断是否按下了enter键*/
csdn.isEnter = function (ev) {
    ev = ev || window.event;
    var code = (ev.keyCode || ev.which);
    return (code == 10 || code == 13);
};

String.prototype.endWith = function (s) {
    return new RegExp("(" + s + ")$", "ig").test(this);
};
String.prototype.trim = function (c) {
    if (!c) c = "\\s";
    return this.replace(new RegExp("(^" + c + "+)|(" + c + "+$)", "ig"), "");
};
String.prototype.trimStart = function () {
    return this.replace(/(^\s+)/g, "");
};
Date.prototype.format = function (fmt) {
    var o = {
        "M+": this.getMonth() + 1,
        "d+": this.getDate(),
        "h+": this.getHours(),
        "m+": this.getMinutes(),
        "s+": this.getSeconds(),
        "q+": Math.floor((this.getMonth() + 3) / 3),
        "S": this.getMilliseconds()
    };
    if (/(y+)/.test(fmt))
        fmt = fmt.replace(RegExp.$1, (this.getFullYear() + "").substr(4 - RegExp.$1.length));
    for (var k in o) {
        if (new RegExp("(" + k + ")").test(fmt))
            fmt = fmt.replace(RegExp.$1, (RegExp.$1.length == 1) ? (o[k]) : (("00" + o[k]).substr(("" + o[k]).length)));
    }
    return fmt;
};
/*验证内容中是否含有外站图片*/
csdn.hasOutImg = function (s, whiteList) {
    if (!whiteList) whiteList = 'csdn.net'.split(',');
    var outImgReg = /<img ([^>]*?)>/ig;
    var m = s.match(outImgReg);
    if (!m) return false;
    for (var i = 0; i < m.length; i++) {
        var src = $(m[i]).attr("src").toLowerCase();
        if (src.length < 7) continue; //不包含http://说明不是站外
        //firefox可以直接拷贝图片信息，等于上传图片
        if (src.substr(0, 5) == 'data:') return true;
        var dm = src.substr(0, src.indexOf('/', 7));
        if (dm.indexOf("http://") == 0) {
            var isOut = true;
            for (var h = 0; h < whiteList.length; h++) {
                if (dm.indexOf(whiteList[h]) != -1) {
                    isOut = false;
                    break;
                }
            }
            if (isOut) return true;
        }
    }
    return false;
};
/*通过文章URL获取ID，传入url或id*/
csdn.getArticleId = function (val) {
    if (val.indexOf("http://") == 0) {
        var mats = /(\/(\d+)\.aspx)|(details\/(\d+))/i.exec(val);
        if (mats) return (mats[0].split('/')[1].split('.')[0]);
    } else if (!isNaN(val)) {
        return (val);
    }
    return 0;
};
/*从cookie获取用户名*/
csdn.getUN = function () {
    var m = document.cookie.match(new RegExp("(^| )UserName=([^;]*)(;|$)"));
    if (m) return m[2];
    else return '';
};
/*从cookie获取用户名*/
csdn.getCookie = function () {
    var m = document.cookie.match(new RegExp("(^| )UserName=([^;]*)(;|$)"));
    if (m) return m[2];
    else return '';
};