var globalData={pageNum:1,pageSize:Number($(".load_more").attr("pageSize"))||10,pageType:"activity",firstLoad:!1,JSONData:[],tabType:"",isLoadMore:!1,totalNum:0,noLoad:!1,tag_Name:""};function getListData(){var a,t=arguments,e=t[0];"index"!=t[2]&&(a=CFG.API_URL+t[2]+"_api?s_title="+tag_Name+"&page="+e),$.ajax({url:a,type:"GET",dataType:"json",success:function(a){setTimeout(function(){$(".load_more").css("display","none")},2e3),"function"==typeof t[t.length-1]&&t[t.length-1](a)}})}function getActiveTab(){var o=window.location.href,a=document.querySelector(".nav_list").getElementsByTagName("li"),t=Array.prototype.slice.call(a),i="";-1!=o.indexOf("?")&&(i=o.split("?")[1].split("=")[1]),(-1<o.indexOf("index")||-1===o.indexOf("blockchain/"))&&$(".list_li").addClass("act_li"),t.map(function(a,t){var e=$(a).attr("action-data");-1<o.indexOf(e)&&(pageNum=globalData.pageNum,pageSize=globalData.pageSize,pageType=e,globalData.pageType=e,globalData.isLoadMore=!1,tag_Name=i,"activity"===e?$(a).addClass("act_li"):"edu"===e?$(a).addClass("act_li"):"spec"===e&&$(a).addClass("act_li"),tabType=globalData.tabType,window.getListData(pageNum,pageSize,e,tabType,tag_Name,function(a){window.bindList(pageType,a)}))})}function bindList(a,t){switch(!t||""!==t.data||"spec"!==a&&"activity"!==a&&"edu"!==a||(t.data={},globalData.noLoad=!0,$("#load_more").text("没有数据了...")),"spec"!==a&&"activity"!==a&&"edu"!==a||(t.data&&t.data.list&&t.data.list.length?(globalData.totalNum=t.data.count,t.data=t.data.list):t.data=[]),globalData.JSONData=globalData.JSONData.concat(t.data),t.data=globalData.JSONData,globalData.firstLoad=!0,globalData.pageNum++,a){case"activity":$("#activity-list .spec-content-ul").html(template("activity_item",t));break;case"spec":$("#theme-list .spec-content-ul").html(template("theme_item",t));break;case"edu":$("#courses-list .spec-content-ul").html(template("courses_item",t))}}function scrollTop(){return Math.max(document.body.scrollTop,document.documentElement.scrollTop)}function documentHeight(){return Math.max(document.body.scrollHeight,document.documentElement.scrollHeight)}function windowHeight(){return"CSS1Compat"==document.compatMode?document.documentElement.clientHeight:document.body.clientHeight}jQuery,window.getActiveTab();var checkTimer,winH=$(window).height();function isView(a){var t=this;if(!a)return!1;var e=this.getElementBottom(a),o=e;return t.scrollTop()<e&&e<t.scrollTop()+t.windowHeight()||t.scrollTop()<o&&o<t.scrollTop()+t.windowHeight()}$(document).scroll(function(){clearTimeout(checkTimer),checkTimer=setTimeout(function(){if(scrollTop()+windowHeight()>=documentHeight()-50&&globalData.firstLoad){var a=globalData.pageNum,t=globalData.pageSize,e=globalData.pageType,o=globalData.tabType;"activity"!==e&&"edu"!==e&&"spec"!==e||(globalData.isLoadMore=!0,$(".load_more").css("display","block"),globalData.noLoad||window.getListData(a,t,e,o,tag_Name,function(a){window.bindList(e,a)}))}},200)});