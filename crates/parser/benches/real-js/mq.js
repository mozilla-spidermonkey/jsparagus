;(function(){
  var result = '000';
  var admckid = '1904182003041421555'
  var urls = ['http://cm.g.doubleclick.net/pixel?google_nid=intel_dmp&google_cm', 'http://cm.pos.baidu.com/pixel?userid=10483045&ext_data=BES', 'http://cm.e.qq.com/cm.fcg?a=2657339418&j=123456&time=1419503821'];
  function __pixel__(idx, v){
    if (v === '1') {
      var img = new Image();
      img.src = urls[idx];
    }
  };
  function __setCookie__(value) {
    if (value) {
      var now = new Date();
      now.setFullYear(now.getFullYear() + 2);
      now = now.toUTCString();
      document.cookie = 'admckid_intel=' + escape(value) + ';domain=.intel.cn;path=/;expires=' + now + ';';
    }
  };
  __setCookie__(admckid);
  
  result = result.split('');
  for (var idx = 0; idx < result.length; idx++) __pixel__(idx, result[idx]);
})()