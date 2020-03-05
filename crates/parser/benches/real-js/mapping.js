;
(function(){
  function setCookie(name, value, Hours) {
    document.cookie = name + "=" + escape(value) + ";path=/;"
  }

  function getCookie(name) {
    var arr = document.cookie.match(new RegExp("(^| )" + name + "=([^;]*)(;|$)"));
    if (arr != null) return unescape(arr[2]);
    return null
  }

  function seriesLoadScripts(scripts, callback) {
    if(typeof(scripts) != "object") var scripts = [scripts];
    var BODY = document.body || document.documentElement;
    var s    = new Array(), last = scripts.length - 1;
    var recursiveLoad = function(i) {
      s[i] = document.createElement("script");
      s[i].setAttribute("type","text/javascript");
      s[i].onload = s[i].onreadystatechange = function() {
        if(!/*@cc_on!@*/0 || this.readyState == "loaded" || this.readyState == "complete") {
          this.onload = this.onreadystatechange = null; this.parentNode.removeChild(this);
          if(i != last) recursiveLoad(i + 1); else if(typeof(callback) == "function") callback();
        }
      }
      s[i].setAttribute("src", scripts[i]);
      BODY.appendChild(s[i]);
    };
    recursiveLoad(0);
  }

  var cookieName = '__adm__';
  var cookie     = getCookie(cookieName);
  var oldFn      = window.onload;

  if(cookie) return;

  window.onload = function(){
    var script = document.createElement('script');
    oldFn && oldFn();
    seriesLoadScripts('//v.admaster.com.cn/m/mq.js', function(){
      setTimeout(function(){
      	setCookie(cookieName, '1');
      }, 3000);
    })
  }
})();