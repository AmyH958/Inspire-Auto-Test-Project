<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_DetailsdjConfig  baseRelativePathinspi_19211e</name>
   <tag></tag>
   <elementGuidId>a3f5dd99-a604-4027-bf2c-70972cc449ce</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//html</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>d7e7ab6d-0e49-475d-afa5-69d555d9f507</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';




































&lt;!--
if (typeof window.event == 'undefined'){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD')|| test_var == 'TEXTAREA'){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == 'INPUT' &amp;&amp; test_type == 'TEXT') ||(test_var == 'INPUT' &amp;&amp; test_type == 'PASSWORD') || test_var == 'TEXTAREA'){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_DatePicker;

function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==''){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/', closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/');
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }

dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
 	
		書目
		館藏
		多媒體
		編輯器
	
	
 
		  
		  
		  
		    
			
			
		  
		


	
	      








 function putAuthorNrToInputField(authorNr) {
		document.getElementById(&quot;inputAuNr&quot;).value = authorNr;
 }
 function clickDelete(idx){
 	var buton = document.getElementById('deleteButon');
 	tapestry.linkOnClick(buton.href+'&amp;sp=l'+idx,'deleteButon', false);
 }
 function clickEdit(idx){
 	var buton = document.getElementById('editButon');
 	tapestry.linkOnClick(buton.href+'&amp;sp=l'+idx,'editButon', false);
 }
 function clickGenerate(idx){
 	var buton = document.getElementById('generateButon');
 	tapestry.linkOnClick(buton.href+'&amp;sp=l'+idx,'generateButon', false);
 }


 









 
  
   
	
	   
	    
	    回到館藏清單
	   
	
   
  
 
 
   
#loading-overlay { position: absolute; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: 0; background-color: transparent; opacity: 0.7; }




























 
 




	 function deleteblock(){
		var parent = document.getElementById('blockDiv');
		var child = document.getElementById('loading-overlay');		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock();
	
	function createPopEdit(href) {
		popupwindow = window.open(&quot;&quot; ,&quot;ManifestareView&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null) popupwindow.opener = self;
		return false;

		}
 









 
  
   條碼號:
   
    
    
     
    

   
   
   
   館藏地:
   
	
    
	    
	       
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dloc = new dTree('window.dloc', 'messages', '/inspireapp/images/'); 
window.dloc.add(0,-1,'館藏地'); 
window.dloc.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement('CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928', 1, true)&quot;); 
window.dloc.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dloc.selectElement('2 - 2', 463, true)&quot;); 
window.dloc.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dloc.selectElement('123 - 123', 583, true)&quot;); 
window.dloc.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dloc.selectElement('20230417 - 20230417', 1183, true)&quot;); 
window.dloc.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dloc.selectElement('20230418 - 20230418', 1203, true)&quot;); 
window.dloc.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement('AH - \\u5B89\\u5357\\u91AB\\u9662', 167, true)&quot;); 
window.dloc.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement('AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340', 177, true)&quot;); 
window.dloc.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dloc.selectElement('av - av', 643, true)&quot;); 
window.dloc.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dloc.selectElement('B007 - B007', 303, true)&quot;); 
window.dloc.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dloc.selectElement('BCSB4 - BCSB4', 883, true)&quot;); 
window.dloc.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dloc.selectElement('BX - \\u53D6\\u66F8\\u6AC31', 823, true)&quot;); 
window.dloc.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dloc.selectElement('BY - \\u53D6\\u66F8\\u6AC32', 903, true)&quot;); 
window.dloc.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dloc.selectElement('CB - \\u5317\\u6E2F\\u5206\\u9928', 169, true)&quot;); 
window.dloc.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 179, true)&quot;); 
window.dloc.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement('BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 180, true)&quot;); 
window.dloc.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 181, true)&quot;); 
window.dloc.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement('BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340', 182, true)&quot;); 
window.dloc.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement('BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB', 183, true)&quot;); 
window.dloc.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dloc.selectElement('cbook - cbook', 623, true)&quot;); 
window.dloc.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dloc.selectElement('circd - circd', 624, true)&quot;); 
window.dloc.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dloc.selectElement('clp - clp', 683, true)&quot;); 
window.dloc.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement('CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662', 170, true)&quot;); 
window.dloc.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement('BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4', 211, true)&quot;); 
window.dloc.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement('CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928', 363, true)&quot;); 
window.dloc.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement('CU - \\u53F0\\u4E2D\\u7E3D\\u9928', 171, true)&quot;); 
window.dloc.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dloc.selectElement('CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)', 603, true)&quot;); 
window.dloc.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 217, true)&quot;); 
window.dloc.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement('MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340', 218, true)&quot;); 
window.dloc.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dloc.selectElement('MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44', 219, true)&quot;); 
window.dloc.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement('MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB', 220, true)&quot;); 
window.dloc.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement('MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 221, true)&quot;); 
window.dloc.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 244, true)&quot;); 
window.dloc.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement('MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340', 245, true)&quot;); 
window.dloc.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dloc.selectElement('MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340', 246, true)&quot;); 
window.dloc.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement('MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340', 248, true)&quot;); 
window.dloc.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 250, true)&quot;); 
window.dloc.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dloc.selectElement('MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340', 251, true)&quot;); 
window.dloc.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dloc.selectElement('MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340', 252, true)&quot;); 
window.dloc.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dloc.selectElement('MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457', 253, true)&quot;); 
window.dloc.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 256, true)&quot;); 
window.dloc.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 257, true)&quot;); 
window.dloc.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 258, true)&quot;); 
window.dloc.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 259, true)&quot;); 
window.dloc.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dloc.selectElement('MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340', 261, true)&quot;); 
window.dloc.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 262, true)&quot;); 
window.dloc.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 263, true)&quot;); 
window.dloc.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dloc.selectElement('MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4', 265, true)&quot;); 
window.dloc.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dloc.selectElement('MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44', 266, true)&quot;); 
window.dloc.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dloc.selectElement('MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3', 267, true)&quot;); 
window.dloc.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement('MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340', 270, true)&quot;); 
window.dloc.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dloc.selectElement('MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8', 271, true)&quot;); 
window.dloc.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement('MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)', 272, true)&quot;); 
window.dloc.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dloc.selectElement('MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4', 273, true)&quot;); 
window.dloc.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement('MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB', 274, true)&quot;); 
window.dloc.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dloc.selectElement('MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3', 275, true)&quot;); 
window.dloc.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dloc.selectElement('new item 7 - new item 7', 1103, true)&quot;); 
window.dloc.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dloc.selectElement('ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599', 276, true)&quot;); 
window.dloc.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dloc.selectElement('e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90', 3, true)&quot;); 
window.dloc.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dloc.selectElement('EB-P - EB-P', 345, true)&quot;); 
window.dloc.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dloc.selectElement('elect - elect', 648, true)&quot;); 
window.dloc.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dloc.selectElement('H-EQ - H-EQ', 343, true)&quot;); 
window.dloc.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dloc.selectElement('H-MR - H-MR', 344, true)&quot;); 
window.dloc.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dloc.selectElement('L - L', 543, true)&quot;); 
window.dloc.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dloc.selectElement('L40 - L40', 863, true)&quot;); 
window.dloc.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement('LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928', 1023, true)&quot;); 
window.dloc.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dloc.selectElement('LB-S - LB-S', 323, true)&quot;); 
window.dloc.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dloc.selectElement('LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3', 173, true)&quot;); 
window.dloc.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement('LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4', 280, true)&quot;); 
window.dloc.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dloc.selectElement('LIB - LIB', 523, true)&quot;); 
window.dloc.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dloc.selectElement('new item 1 - new item 1', 423, true)&quot;); 
window.dloc.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dloc.selectElement('new item 3 - new item 3', 484, true)&quot;); 
window.dloc.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dloc.selectElement('new item 10 - new item 10', 1283, true)&quot;); 
window.dloc.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dloc.selectElement('new item 12 - new item 12', 1323, true)&quot;); 
window.dloc.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dloc.selectElement('new item 13 - new item 13', 1343, true)&quot;); 
window.dloc.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dloc.selectElement('new item 14 - new item 14', 1344, true)&quot;); 
window.dloc.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dloc.selectElement('new item 16 - new item 16', 1264, true)&quot;); 
window.dloc.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dloc.selectElement('new item 2 - new item 2', 483, true)&quot;); 
window.dloc.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dloc.selectElement('new item 20 - new item 20', 1425, true)&quot;); 
window.dloc.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dloc.selectElement('new item 4 - new item 4', 943, true)&quot;); 
window.dloc.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dloc.selectElement('new item 5 - new item 5', 963, true)&quot;); 
window.dloc.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dloc.selectElement('new item 6 - \\u82F1\\u624D\\u6821\\u5340', 1063, true)&quot;); 
window.dloc.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dloc.selectElement('new item 8 - new item 8', 1243, true)&quot;); 
window.dloc.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dloc.selectElement('new item 9 - new item 9', 1263, true)&quot;); 
window.dloc.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dloc.selectElement('NPTU - NPTU', 1043, true)&quot;); 
window.dloc.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dloc.selectElement('OUK - OUK', 503, true)&quot;); 
window.dloc.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement('PT - \\u57F9\\u5FB7\\u91AB\\u9662', 174, true)&quot;); 
window.dloc.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dloc.selectElement('new item 11 - new item 11', 1303, true)&quot;); 
window.dloc.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dloc.selectElement('new item 17 - new item 17', 1363, true)&quot;); 
window.dloc.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement('PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340', 283, true)&quot;); 
window.dloc.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dloc.selectElement('ptext - ptext', 645, true)&quot;); 
window.dloc.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dloc.selectElement('SB3 - SB3', 1083, true)&quot;); 
window.dloc.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dloc.selectElement('T-P - T-P', 324, true)&quot;); 
window.dloc.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dloc.selectElement('TBBK - TBBK', 1403, true)&quot;); 
window.dloc.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement('TH - \\u53F0\\u5317\\u5206\\u9662', 175, true)&quot;); 
window.dloc.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement('THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340', 284, true)&quot;); 
window.dloc.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement('THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340', 285, true)&quot;); 
window.dloc.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dloc.selectElement('W-P - W-P', 325, true)&quot;); 
window.dloc.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement('YH - \\u8C50\\u539F\\u5206\\u9662', 176, true)&quot;); 
window.dloc.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dloc.selectElement('new item 18 - new item 18', 1423, true)&quot;); 
window.dloc.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dloc.selectElement('new item 19 - new item 19', 1424, true)&quot;); 
window.dloc.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement('YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340', 286, true)&quot;); 
window.dloc.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dloc.selectElement('z3llc - z3llc', 983, true)&quot;); 
window.dloc.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dloc.selectElement('z6bkf - z6bkf', 647, true)&quot;); 
window.dloc.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dloc.selectElement('zd1a2 - zd1a2', 646, true)&quot;); 
window.dloc.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dloc.selectElement('zd1e - zd1e', 663, true)&quot;); 
window.dloc.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dloc.selectElement('zdlf - zdlf', 644, true)&quot;); 
window.dloc.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement('\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340', 403, true)&quot;); 
window.dloc.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dloc.selectElement('\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF', 563, true)&quot;); 
window.dloc.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement('\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB', 1383, true)&quot;); 
window.dloc.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement('\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928', 383, true)&quot;); 
window.dloc.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement('\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340', 1384, true)&quot;); 
window.dloc.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement('\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928', 443, true)&quot;); 
window.dloc.selectElement = function(lname, id, hideTree) { 
document.getElementById('loc_0').value = id; 
document.getElementById('elementName').value = lname; 
if(hideTree == true) changeStatus('locTree'); 
if(lname) { tapestry.linkOnClick(document.getElementById('selectLink').href+'?sp=l'+id,'selectLink', false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById('selectLink').href+'?sp=l-1','selectLink', false); 
 } 
}; 
 document.getElementById('locArea').innerHTML =  window.dloc; 
  
  
  


	    
    
   
  
  
   資料類型:
   
BD-藍光光碟
CA-靜畫資料
DB-資料庫
DF-磁片
DO-電子書
EA-立體模型
EB-線上電子書
EJ-線上電子期刊
EP-電子期刊光碟
ERROR-有問題特藏
FA-磁帶
KT-多媒體組件
LA-地圖
LD-影碟
MP-MP3
NH-微縮單片
NR-微縮捲片
QA-地球儀
R-參考書
SL-幻燈片
VC-錄影帶
VD-VCD
BOX-書箱
ERM_DB-電子資料庫(ERM)
ERM_WS-網路資源(ERM)
ERM_EB-電子書(ERM)
ERM_EJ-電子期刊(ERM)
XL-X-ray
BOOK-圖書
AC-錄音帶
APP-附件
P-現期期刊
ac-ac
DD-影像光碟(DVD)
MD-行動設備
S-裝訂期刊
AD-唱片
CD-光碟
booklet-小冊子
KKtest-KK
YYtest2-YY2
YYtest4-YYYtest
0425-0425
TEST-TEST
TEST0425-TEST0425

   價格:
   
  
   
   館藏流通類別:
   
	
    
     	
B可借圖書
B電子資源
eee
M可借行動設備
P可借期刊
V可借視聽
www
不流通
書箱借閱30天

    
   
      	採購實價:
   		
   				
   				
   			
   			
   		
	
  
   條碼號類別:
   
     
      電子書
     
   
	  附件:
   
	
	  
  
  
    分類方法:
    

中文圖書分類法
美國國會圖書分類法
杜威分類法
美國國家醫學圖書館分類法
何日章中國圖書十進分類法

	 特藏:
	
測試2
測試3
PickListValues.15007340
PickListValues.15007339
PickListValues.15007338
PickListValues.15007337
商務企劃專案實務
PickListValues.15007336
測試0
通用特藏
禮物
測試4
電腦動畫視覺特效實務
電腦輔助繪圖
測試1
一般藏書
贈書
中英筆譯(二)
英語會話(二)
基礎寫作(二)
進階英語聽講練習(二)
進階實用英文寫作(二)
寫作入門(二)
專題研究方法
第二外語 (二) –日文
第二外語 (二) –西班牙文
第二外語聽講練習(二) –日文
第二外語聽講練習(二) –西班牙文
多媒體語言學習(二)
表演藝術(二)
語言測驗檢析(二)
閱讀與簡報(二)
實用英文寫作(二)
機械實務設計
測試5

	   
     
  
    分類號: 
    
	    
	    	
	    
    
	 分類法版本:
   
 
  
  
    作者號規則
    
      

科特號
何日章著者號
科特號(T2)

    
	館藏狀態:
	
	  
	  
	   
在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

	  
	 
	
  
  
    作者名字
    
     
       
    
    
	期刊架位號:
	
		
	  
  
  
   
  
    作者號
    
      
    
        
        


      
    
	   到館日期:
    
	  
  
    
  
  
   出版年:
   
	 卷號:
   
 
  

  
   索書號:
   
   		
   					
  
   					
			   			
			   		
			   		[AuthorNumber] +[ItemCollection] +[Year] +[ClassificationNumber] +[MaterialType] +[VolumeNumber] +[CopyNumber] 
   		
   
   複本號:   
	
    
    
     	
    
   

  
  
  
	於公共目錄顯示:
	
	館藏流通狀態:
	在架
	
   
  
  
   使用類型:
   
eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試
  
    電子資源連結:
   
  

  
   館員備註:
   (限1000字元)
   
   
  
  
   採購備註:
   
   備註欄:
   
 
   
  
   期刊備註:
   
   
   流通註記:
   
  
   
   
   其他號碼:
   
   
   
   
    最後盤點日期:
   
   
  
  
   
   
   財產編號:
   
   
   
   
   
   館藏經費:
   
   
    
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏經費2020 - 圖書 - -2001文學院-圖書費2020 - 圖書 - -2002商學院-圖書費2020 - 圖書 - -ah01安南醫院經費-2021 - 圖書 - -091文學院-2021 - 圖書 - -cwen1圖書費教育部補助2021 - 圖書 - -cwen101兒童書教育部補助2021 - 圖書 - -cwen102教科書教育部補助2021 - 圖書 - 20210825TEST20210825TEST-2021 - 圖書 - *new cost-2021 - 圖書 - -e2enew cost-2021 - 圖書 - GOGO-2021 - 視聽資料 - -cwen3DVD採購案學校補助2021 - 視聽資料 - -cwen31卡通影片學校補助2021 - 圖書 - -cwen311卡通電影學校補助2021 - 圖書 - NickBook尼克圖書經費人文社會學系2021 - 圖書 - -2021ah安南醫院經費學校款項2021 - 期刊 - J2021圖書館經費教育部補助2022 - 圖書 - JB01童書經費圖書館2023 - 圖書 - TEST234new cost-2023 - 圖書 - TEST111new cost-
  
  
window.ditemCostTree = new dTree('window.ditemCostTree', 'messages', '/inspireapp/images/'); 
window.ditemCostTree.add(0,-1,'館藏經費'); 
window.ditemCostTree.add(2502,0,&quot;2020 - 圖書 - -2001文學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement('2020 - \u5716\u66F8 - -2001\u6587\u5B78\u9662-\u5716\u66F8\u8CBB', 2502, true)&quot;); 
window.ditemCostTree.add(2525,0,&quot;2020 - 圖書 - -2002商學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement('2020 - \u5716\u66F8 - -2002\u5546\u5B78\u9662-\u5716\u66F8\u8CBB', 2525, true)&quot;); 
window.ditemCostTree.add(2623,0,&quot;2020 - 圖書 - -ah01安南醫院經費-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2020 - \u5716\u66F8 - -ah01\u5B89\u5357\u91AB\u9662\u7D93\u8CBB-', 2623, true)&quot;); 
window.ditemCostTree.add(3161,0,&quot;2021 - 圖書 - -091文學院-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -091\u6587\u5B78\u9662-', 3161, true)&quot;); 
window.ditemCostTree.add(2661,0,&quot;2021 - 圖書 - -cwen1圖書費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -cwen1\u5716\u66F8\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9', 2661, true)&quot;); 
window.ditemCostTree.add(2681,2661,&quot;2021 - 圖書 - -cwen101兒童書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -cwen101\u5152\u7AE5\u66F8\u6559\u80B2\u90E8\u88DC\u52A9', 2681, true)&quot;); 
window.ditemCostTree.add(2682,2661,&quot;2021 - 圖書 - -cwen102教科書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -cwen102\u6559\u79D1\u66F8\u6559\u80B2\u90E8\u88DC\u52A9', 2682, true)&quot;); 
window.ditemCostTree.add(3441,0,&quot;2021 - 圖書 - 20210825TEST20210825TEST-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - 20210825TEST20210825TEST-', 3441, true)&quot;); 
window.ditemCostTree.add(3421,0,&quot;2021 - 圖書 - *new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - *new cost-', 3421, true)&quot;); 
window.ditemCostTree.add(3461,0,&quot;2021 - 圖書 - -e2enew cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -e2enew cost-', 3461, true)&quot;); 
window.ditemCostTree.add(3081,0,&quot;2021 - 圖書 - GOGO-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - GOGO-', 3081, true)&quot;); 
window.ditemCostTree.add(2683,0,&quot;2021 - 視聽資料 - -cwen3DVD採購案學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u8996\u807D\u8CC7\u6599 - -cwen3DVD\u63A1\u8CFC\u6848\u5B78\u6821\u88DC\u52A9', 2683, true)&quot;); 
window.ditemCostTree.add(2721,2683,&quot;2021 - 視聽資料 - -cwen31卡通影片學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u8996\u807D\u8CC7\u6599 - -cwen31\u5361\u901A\u5F71\u7247\u5B78\u6821\u88DC\u52A9', 2721, true)&quot;); 
window.ditemCostTree.add(2781,2721,&quot;2021 - 圖書 - -cwen311卡通電影學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -cwen311\u5361\u901A\u96FB\u5F71\u5B78\u6821\u88DC\u52A9', 2781, true)&quot;); 
window.ditemCostTree.add(3261,0,&quot;2021 - 圖書 - NickBook尼克圖書經費人文社會學系&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - NickBook\u5C3C\u514B\u5716\u66F8\u7D93\u8CBB\u4EBA\u6587\u793E\u6703\u5B78\u7CFB', 3261, true)&quot;); 
window.ditemCostTree.add(2801,0,&quot;2021 - 圖書 - -2021ah安南醫院經費學校款項&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u5716\u66F8 - -2021ah\u5B89\u5357\u91AB\u9662\u7D93\u8CBB\u5B78\u6821\u6B3E\u9805', 2801, true)&quot;); 
window.ditemCostTree.add(3221,0,&quot;2021 - 期刊 - J2021圖書館經費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement('2021 - \u671F\u520A - J2021\u5716\u66F8\u9928\u7D93\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9', 3221, true)&quot;); 
window.ditemCostTree.add(3121,0,&quot;2022 - 圖書 - JB01童書經費圖書館&quot;, &quot;javascript:window.ditemCostTree.selectElement('2022 - \u5716\u66F8 - JB01\u7AE5\u66F8\u7D93\u8CBB\u5716\u66F8\u9928', 3121, true)&quot;); 
window.ditemCostTree.add(3641,0,&quot;2023 - 圖書 - TEST234new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2023 - \u5716\u66F8 - TEST234new cost-', 3641, true)&quot;); 
window.ditemCostTree.add(3601,0,&quot;2023 - 圖書 - TEST111new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement('2023 - \u5716\u66F8 - TEST111new cost-', 3601, true)&quot;); 
window.ditemCostTree.selectElement = function(lname, id, hideTree) { 
document.getElementById('itemCostTree_0').value = id; 
document.getElementById('elementName_0').value = lname; 
if(hideTree == true) changeStatus('itemCostTreeTree'); 
}; 
 document.getElementById('itemCostTreeArea').innerHTML =  window.ditemCostTree; 
  
  
  


   
   
  
   
   	
   	 	新增日期:
   	 	
   	
  		2021/03/23 15:32:03
   	
   	
   	
   	
   
 
 
 
 
 
 
 
 



 



	
     
       
       
         修改/存檔
       
     
     
     取消
     
  





  
	館藏數量:
	 
  
  
	產生方式:
	
	  
	   
	    自動產生
	    手動輸入
	   
	  
	
  

   

 
  
    產生
  
 









 
    
 
  
    Author nr
  
  
 
 
  
 

 


	clearCallNumberItem();
	clearPickListValue();
	setCallNumberItem('null', 'AuthorNumber', '', ' ', 'true'); setCallNumberItem('null', 'ItemCollection', '', '  ', 'true'); setCallNumberItem('null', 'Year', '', '  ', 'true'); setCallNumberItem('null', 'ClassificationNumber', '', ' ', 'true'); setCallNumberItem('null', 'MaterialType', '', '  ', 'true'); setCallNumberItem('null', 'VolumeNumber', '', '  ', 'true'); setCallNumberItem('null', 'CopyNumber', '', ' ', 'true'); 
	var isOmitIfmaterialTypeBook = true
	setCallNumber(getCallNumberItem(), isOmitIfmaterialTypeBook);
	setPickListValue('0', 'BD'); setPickListValue('1', 'CA'); setPickListValue('2', 'DB'); setPickListValue('3', 'DF'); setPickListValue('4', 'DO'); setPickListValue('5', 'EA'); setPickListValue('6', 'EB'); setPickListValue('7', 'EJ'); setPickListValue('8', 'EP'); setPickListValue('9', 'ERROR'); setPickListValue('10', 'FA'); setPickListValue('11', 'KT'); setPickListValue('12', 'LA'); setPickListValue('13', 'LD'); setPickListValue('14', 'MP'); setPickListValue('15', 'NH'); setPickListValue('16', 'NR'); setPickListValue('17', 'QA'); setPickListValue('18', 'R'); setPickListValue('19', 'SL'); setPickListValue('20', 'VC'); setPickListValue('21', 'VD'); setPickListValue('22', 'BOX'); setPickListValue('23', 'ERM_DB'); setPickListValue('24', 'ERM_WS'); setPickListValue('25', 'ERM_EB'); setPickListValue('26', 'ERM_EJ'); setPickListValue('27', 'XL'); setPickListValue('28', 'BOOK'); setPickListValue('29', 'AC'); setPickListValue('30', 'APP'); setPickListValue('31', 'P'); setPickListValue('32', 'ac'); setPickListValue('33', 'DD'); setPickListValue('34', 'MD'); setPickListValue('35', 'S'); setPickListValue('36', 'AD'); setPickListValue('37', 'CD'); setPickListValue('38', 'booklet'); setPickListValue('39', 'KKtest'); setPickListValue('40', 'YYtest2'); setPickListValue('41', 'YYtest4'); setPickListValue('42', '0425'); setPickListValue('43', 'TEST'); setPickListValue('44', 'TEST0425'); 
	doCallNumber();


 




	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;editForm&quot;);
calendar_DatePicker = new Calendar(1616428800000);
	
calendar_DatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_DatePicker.onchange = function() {
  var field = tapestry.byId(&quot;editForm&quot;).DatePicker;
  var value = calendar_DatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }

closeDialogComponent('AuthornrDialog');
try {
     document.getElementById(&quot;generateArea&quot;).style.display ='none';
    }catch(e) {}
closeDialogComponent('CloseReminderDialog');
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById('ajaxLoaderDiv');
            	var processDiv = document.getElementById('pressingDiv');
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById('pressingDiv');
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent('TinreadErrorDialog');
closeDialogComponent('TinreadMessageDialog');});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六 12345678910111213141516171819202122232425262728293031          15 三月, 2024Clear</value>
      <webElementGuid>5e17ad81-e7c3-479c-be19-d2f735abccb0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>150e2522-5ff1-4fd4-a8c8-e3f447563ea1</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>7ecbd0f7-1f68-4854-a4fc-2aeb6c7a8d9a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;




































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_DatePicker;

function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }

dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
 	
		書目
		館藏
		多媒體
		編輯器
	
	
 
		  
		  
		  
		    
			
			
		  
		


	
	      








 function putAuthorNrToInputField(authorNr) {
		document.getElementById(&quot;inputAuNr&quot;).value = authorNr;
 }
 function clickDelete(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;deleteButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;deleteButon&quot; , &quot;'&quot; , &quot;, false);
 }
 function clickEdit(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;editButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;editButon&quot; , &quot;'&quot; , &quot;, false);
 }
 function clickGenerate(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;generateButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;generateButon&quot; , &quot;'&quot; , &quot;, false);
 }


 









 
  
   
	
	   
	    
	    回到館藏清單
	   
	
   
  
 
 
   
#loading-overlay { position: absolute; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: 0; background-color: transparent; opacity: 0.7; }




























 
 




	 function deleteblock(){
		var parent = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv&quot; , &quot;'&quot; , &quot;);
		var child = document.getElementById(&quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;);		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock();
	
	function createPopEdit(href) {
		popupwindow = window.open(&quot;&quot; ,&quot;ManifestareView&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null) popupwindow.opener = self;
		return false;

		}
 









 
  
   條碼號:
   
    
    
     
    

   
   
   
   館藏地:
   
	
    
	    
	       
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dloc = new dTree(&quot; , &quot;'&quot; , &quot;window.dloc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dloc.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dloc.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dloc.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dloc.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dloc.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dloc.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dloc.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dloc.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dloc.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dloc.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dloc.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dloc.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dloc.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dloc.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dloc.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dloc.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dloc.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dloc.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dloc.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dloc.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dloc.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dloc.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dloc.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dloc.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dloc.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dloc.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dloc.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dloc.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dloc.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dloc.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dloc.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dloc.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dloc.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dloc.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dloc.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dloc.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dloc.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dloc.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dloc.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dloc.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dloc.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dloc.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dloc.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dloc.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dloc.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dloc.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dloc.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dloc.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dloc.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dloc.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dloc.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dloc.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dloc.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dloc.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dloc.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dloc.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dloc.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dloc.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dloc.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dloc.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dloc.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dloc.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dloc.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dloc.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dloc.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dloc.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dloc.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dloc.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dloc.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dloc.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dloc.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dloc.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dloc.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dloc.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dloc.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dloc.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dloc.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dloc.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dloc.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dloc.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dloc.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dloc.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dloc.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dloc.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dloc.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dloc.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dloc.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dloc.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dloc.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dloc.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dloc.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dloc.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dloc.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dloc.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dloc.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dloc.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dloc.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dloc.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dloc.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dloc.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dloc.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dloc.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dloc.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dloc.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dloc.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dloc.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dloc.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dloc.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dloc.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dloc.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dloc.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dloc.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dloc.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dloc.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;loc_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;locTree&quot; , &quot;'&quot; , &quot;); 
if(lname) { tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l&quot; , &quot;'&quot; , &quot;+id,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l-1&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;locArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dloc; 
  
  
  


	    
    
   
  
  
   資料類型:
   
BD-藍光光碟
CA-靜畫資料
DB-資料庫
DF-磁片
DO-電子書
EA-立體模型
EB-線上電子書
EJ-線上電子期刊
EP-電子期刊光碟
ERROR-有問題特藏
FA-磁帶
KT-多媒體組件
LA-地圖
LD-影碟
MP-MP3
NH-微縮單片
NR-微縮捲片
QA-地球儀
R-參考書
SL-幻燈片
VC-錄影帶
VD-VCD
BOX-書箱
ERM_DB-電子資料庫(ERM)
ERM_WS-網路資源(ERM)
ERM_EB-電子書(ERM)
ERM_EJ-電子期刊(ERM)
XL-X-ray
BOOK-圖書
AC-錄音帶
APP-附件
P-現期期刊
ac-ac
DD-影像光碟(DVD)
MD-行動設備
S-裝訂期刊
AD-唱片
CD-光碟
booklet-小冊子
KKtest-KK
YYtest2-YY2
YYtest4-YYYtest
0425-0425
TEST-TEST
TEST0425-TEST0425

   價格:
   
  
   
   館藏流通類別:
   
	
    
     	
B可借圖書
B電子資源
eee
M可借行動設備
P可借期刊
V可借視聽
www
不流通
書箱借閱30天

    
   
      	採購實價:
   		
   				
   				
   			
   			
   		
	
  
   條碼號類別:
   
     
      電子書
     
   
	  附件:
   
	
	  
  
  
    分類方法:
    

中文圖書分類法
美國國會圖書分類法
杜威分類法
美國國家醫學圖書館分類法
何日章中國圖書十進分類法

	 特藏:
	
測試2
測試3
PickListValues.15007340
PickListValues.15007339
PickListValues.15007338
PickListValues.15007337
商務企劃專案實務
PickListValues.15007336
測試0
通用特藏
禮物
測試4
電腦動畫視覺特效實務
電腦輔助繪圖
測試1
一般藏書
贈書
中英筆譯(二)
英語會話(二)
基礎寫作(二)
進階英語聽講練習(二)
進階實用英文寫作(二)
寫作入門(二)
專題研究方法
第二外語 (二) –日文
第二外語 (二) –西班牙文
第二外語聽講練習(二) –日文
第二外語聽講練習(二) –西班牙文
多媒體語言學習(二)
表演藝術(二)
語言測驗檢析(二)
閱讀與簡報(二)
實用英文寫作(二)
機械實務設計
測試5

	   
     
  
    分類號: 
    
	    
	    	
	    
    
	 分類法版本:
   
 
  
  
    作者號規則
    
      

科特號
何日章著者號
科特號(T2)

    
	館藏狀態:
	
	  
	  
	   
在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

	  
	 
	
  
  
    作者名字
    
     
       
    
    
	期刊架位號:
	
		
	  
  
  
   
  
    作者號
    
      
    
        
        


      
    
	   到館日期:
    
	  
  
    
  
  
   出版年:
   
	 卷號:
   
 
  

  
   索書號:
   
   		
   					
  
   					
			   			
			   		
			   		[AuthorNumber] +[ItemCollection] +[Year] +[ClassificationNumber] +[MaterialType] +[VolumeNumber] +[CopyNumber] 
   		
   
   複本號:   
	
    
    
     	
    
   

  
  
  
	於公共目錄顯示:
	
	館藏流通狀態:
	在架
	
   
  
  
   使用類型:
   
eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試
  
    電子資源連結:
   
  

  
   館員備註:
   (限1000字元)
   
   
  
  
   採購備註:
   
   備註欄:
   
 
   
  
   期刊備註:
   
   
   流通註記:
   
  
   
   
   其他號碼:
   
   
   
   
    最後盤點日期:
   
   
  
  
   
   
   財產編號:
   
   
   
   
   
   館藏經費:
   
   
    
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏經費2020 - 圖書 - -2001文學院-圖書費2020 - 圖書 - -2002商學院-圖書費2020 - 圖書 - -ah01安南醫院經費-2021 - 圖書 - -091文學院-2021 - 圖書 - -cwen1圖書費教育部補助2021 - 圖書 - -cwen101兒童書教育部補助2021 - 圖書 - -cwen102教科書教育部補助2021 - 圖書 - 20210825TEST20210825TEST-2021 - 圖書 - *new cost-2021 - 圖書 - -e2enew cost-2021 - 圖書 - GOGO-2021 - 視聽資料 - -cwen3DVD採購案學校補助2021 - 視聽資料 - -cwen31卡通影片學校補助2021 - 圖書 - -cwen311卡通電影學校補助2021 - 圖書 - NickBook尼克圖書經費人文社會學系2021 - 圖書 - -2021ah安南醫院經費學校款項2021 - 期刊 - J2021圖書館經費教育部補助2022 - 圖書 - JB01童書經費圖書館2023 - 圖書 - TEST234new cost-2023 - 圖書 - TEST111new cost-
  
  
window.ditemCostTree = new dTree(&quot; , &quot;'&quot; , &quot;window.ditemCostTree&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ditemCostTree.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏經費&quot; , &quot;'&quot; , &quot;); 
window.ditemCostTree.add(2502,0,&quot;2020 - 圖書 - -2001文學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -2001\u6587\u5B78\u9662-\u5716\u66F8\u8CBB&quot; , &quot;'&quot; , &quot;, 2502, true)&quot;); 
window.ditemCostTree.add(2525,0,&quot;2020 - 圖書 - -2002商學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -2002\u5546\u5B78\u9662-\u5716\u66F8\u8CBB&quot; , &quot;'&quot; , &quot;, 2525, true)&quot;); 
window.ditemCostTree.add(2623,0,&quot;2020 - 圖書 - -ah01安南醫院經費-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -ah01\u5B89\u5357\u91AB\u9662\u7D93\u8CBB-&quot; , &quot;'&quot; , &quot;, 2623, true)&quot;); 
window.ditemCostTree.add(3161,0,&quot;2021 - 圖書 - -091文學院-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -091\u6587\u5B78\u9662-&quot; , &quot;'&quot; , &quot;, 3161, true)&quot;); 
window.ditemCostTree.add(2661,0,&quot;2021 - 圖書 - -cwen1圖書費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen1\u5716\u66F8\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2661, true)&quot;); 
window.ditemCostTree.add(2681,2661,&quot;2021 - 圖書 - -cwen101兒童書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen101\u5152\u7AE5\u66F8\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2681, true)&quot;); 
window.ditemCostTree.add(2682,2661,&quot;2021 - 圖書 - -cwen102教科書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen102\u6559\u79D1\u66F8\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2682, true)&quot;); 
window.ditemCostTree.add(3441,0,&quot;2021 - 圖書 - 20210825TEST20210825TEST-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - 20210825TEST20210825TEST-&quot; , &quot;'&quot; , &quot;, 3441, true)&quot;); 
window.ditemCostTree.add(3421,0,&quot;2021 - 圖書 - *new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - *new cost-&quot; , &quot;'&quot; , &quot;, 3421, true)&quot;); 
window.ditemCostTree.add(3461,0,&quot;2021 - 圖書 - -e2enew cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -e2enew cost-&quot; , &quot;'&quot; , &quot;, 3461, true)&quot;); 
window.ditemCostTree.add(3081,0,&quot;2021 - 圖書 - GOGO-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - GOGO-&quot; , &quot;'&quot; , &quot;, 3081, true)&quot;); 
window.ditemCostTree.add(2683,0,&quot;2021 - 視聽資料 - -cwen3DVD採購案學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u8996\u807D\u8CC7\u6599 - -cwen3DVD\u63A1\u8CFC\u6848\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2683, true)&quot;); 
window.ditemCostTree.add(2721,2683,&quot;2021 - 視聽資料 - -cwen31卡通影片學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u8996\u807D\u8CC7\u6599 - -cwen31\u5361\u901A\u5F71\u7247\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2721, true)&quot;); 
window.ditemCostTree.add(2781,2721,&quot;2021 - 圖書 - -cwen311卡通電影學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen311\u5361\u901A\u96FB\u5F71\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2781, true)&quot;); 
window.ditemCostTree.add(3261,0,&quot;2021 - 圖書 - NickBook尼克圖書經費人文社會學系&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - NickBook\u5C3C\u514B\u5716\u66F8\u7D93\u8CBB\u4EBA\u6587\u793E\u6703\u5B78\u7CFB&quot; , &quot;'&quot; , &quot;, 3261, true)&quot;); 
window.ditemCostTree.add(2801,0,&quot;2021 - 圖書 - -2021ah安南醫院經費學校款項&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -2021ah\u5B89\u5357\u91AB\u9662\u7D93\u8CBB\u5B78\u6821\u6B3E\u9805&quot; , &quot;'&quot; , &quot;, 2801, true)&quot;); 
window.ditemCostTree.add(3221,0,&quot;2021 - 期刊 - J2021圖書館經費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u671F\u520A - J2021\u5716\u66F8\u9928\u7D93\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 3221, true)&quot;); 
window.ditemCostTree.add(3121,0,&quot;2022 - 圖書 - JB01童書經費圖書館&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2022 - \u5716\u66F8 - JB01\u7AE5\u66F8\u7D93\u8CBB\u5716\u66F8\u9928&quot; , &quot;'&quot; , &quot;, 3121, true)&quot;); 
window.ditemCostTree.add(3641,0,&quot;2023 - 圖書 - TEST234new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2023 - \u5716\u66F8 - TEST234new cost-&quot; , &quot;'&quot; , &quot;, 3641, true)&quot;); 
window.ditemCostTree.add(3601,0,&quot;2023 - 圖書 - TEST111new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2023 - \u5716\u66F8 - TEST111new cost-&quot; , &quot;'&quot; , &quot;, 3601, true)&quot;); 
window.ditemCostTree.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;itemCostTree_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_0&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;itemCostTreeTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;itemCostTreeArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ditemCostTree; 
  
  
  


   
   
  
   
   	
   	 	新增日期:
   	 	
   	
  		2021/03/23 15:32:03
   	
   	
   	
   	
   
 
 
 
 
 
 
 
 



 



	
     
       
       
         修改/存檔
       
     
     
     取消
     
  





  
	館藏數量:
	 
  
  
	產生方式:
	
	  
	   
	    自動產生
	    手動輸入
	   
	  
	
  

   

 
  
    產生
  
 









 
    
 
  
    Author nr
  
  
 
 
  
 

 


	clearCallNumberItem();
	clearPickListValue();
	setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AuthorNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ItemCollection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Year&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ClassificationNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MaterialType&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VolumeNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CopyNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); 
	var isOmitIfmaterialTypeBook = true
	setCallNumber(getCallNumberItem(), isOmitIfmaterialTypeBook);
	setPickListValue(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DF&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DO&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;7&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EJ&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;8&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;9&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERROR&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KT&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;12&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;13&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;15&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NH&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;16&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NR&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;17&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;18&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;R&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SL&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VC&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;21&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BOX&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;23&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_DB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;24&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_WS&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_EB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;26&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_EJ&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;XL&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;28&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BOOK&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;29&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AC&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;30&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;APP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;31&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;P&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;32&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ac&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;33&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;34&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;35&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;S&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;36&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;37&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;38&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;booklet&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;39&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KKtest&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;40&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;YYtest2&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;41&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;YYtest4&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;42&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;0425&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;43&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TEST&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;44&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TEST0425&quot; , &quot;'&quot; , &quot;); 
	doCallNumber();


 




	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;editForm&quot;);
calendar_DatePicker = new Calendar(1616428800000);
	
calendar_DatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_DatePicker.onchange = function() {
  var field = tapestry.byId(&quot;editForm&quot;).DatePicker;
  var value = calendar_DatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }

closeDialogComponent(&quot; , &quot;'&quot; , &quot;AuthornrDialog&quot; , &quot;'&quot; , &quot;);
try {
     document.getElementById(&quot;generateArea&quot;).style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六 12345678910111213141516171819202122232425262728293031          15 三月, 2024Clear&quot;) or . = concat(&quot;



Details

		
		
djConfig = {&quot;baseRelativePath&quot;:&quot;/inspireapp/assets/static/dojo-0.4.3-custom-4.1.6/&quot;,&quot;preventBackButtonFix&quot;:false,&quot;parseWidgets&quot;:true,&quot;locale&quot;:&quot;zh-tw&quot;} 

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;




































&lt;!--
if (typeof window.event == &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){ 
	document.onkeypress = function(e){ 
		var test_var=e.target.nodeName.toUpperCase(); 
		if (e.target.type) 
				var test_type=e.target.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)|| test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
				return e.keyCode; 
			}
		else if (e.keyCode == 8){ 
				e.preventDefault(); } } 
	}else{ 
		document.onkeydown = function(){ 
			var test_var=event.srcElement.tagName.toUpperCase(); 
			if (event.srcElement.type) 
				var test_type=event.srcElement.type.toUpperCase(); 
		if ((test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;) ||(test_var == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot; &amp;&amp; test_type == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;) || test_var == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;){ 
			return event.keyCode; 
			}else if (event.keyCode == 8){ 
				event.returnValue=false; 
				} } }
var calendar_DatePicker;

function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }

dojo.require(&quot;tapestry.fx&quot;);
// -->






 
 
 
	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which;
		}else{
			var keyNum = e;
		}
		
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				popupwindow = window.open(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;,&quot;UtilizatorPhraseDialog&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
				popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
				popupwindow.location = href;
	
				if (popupwindow == null) popupwindow.opener = self;
				return false;
			}
		}
	}

 

	

	
		
		  
		    
 	
		書目
		館藏
		多媒體
		編輯器
	
	
 
		  
		  
		  
		    
			
			
		  
		


	
	      








 function putAuthorNrToInputField(authorNr) {
		document.getElementById(&quot;inputAuNr&quot;).value = authorNr;
 }
 function clickDelete(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;deleteButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;deleteButon&quot; , &quot;'&quot; , &quot;, false);
 }
 function clickEdit(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;editButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;editButon&quot; , &quot;'&quot; , &quot;, false);
 }
 function clickGenerate(idx){
 	var buton = document.getElementById(&quot; , &quot;'&quot; , &quot;generateButon&quot; , &quot;'&quot; , &quot;);
 	tapestry.linkOnClick(buton.href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+idx,&quot; , &quot;'&quot; , &quot;generateButon&quot; , &quot;'&quot; , &quot;, false);
 }


 









 
  
   
	
	   
	    
	    回到館藏清單
	   
	
   
  
 
 
   
#loading-overlay { position: absolute; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: 0; background-color: transparent; opacity: 0.7; }




























 
 




	 function deleteblock(){
		var parent = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv&quot; , &quot;'&quot; , &quot;);
		var child = document.getElementById(&quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;);		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock();
	
	function createPopEdit(href) {
		popupwindow = window.open(&quot;&quot; ,&quot;ManifestareView&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null) popupwindow.opener = self;
		return false;

		}
 









 
  
   條碼號:
   
    
    
     
    

   
   
   
   館藏地:
   
	
    
	    
	       
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dloc = new dTree(&quot; , &quot;'&quot; , &quot;window.dloc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dloc.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dloc.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dloc.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dloc.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dloc.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dloc.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dloc.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dloc.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dloc.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dloc.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dloc.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dloc.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dloc.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dloc.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dloc.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dloc.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dloc.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dloc.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dloc.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dloc.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dloc.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dloc.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dloc.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dloc.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dloc.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dloc.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dloc.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dloc.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dloc.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dloc.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dloc.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dloc.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dloc.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dloc.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dloc.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dloc.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dloc.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dloc.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dloc.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dloc.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dloc.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dloc.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dloc.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dloc.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dloc.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dloc.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dloc.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dloc.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dloc.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dloc.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dloc.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dloc.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dloc.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dloc.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dloc.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dloc.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dloc.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dloc.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dloc.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dloc.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dloc.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dloc.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dloc.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dloc.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dloc.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dloc.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dloc.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dloc.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dloc.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dloc.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dloc.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dloc.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dloc.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dloc.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dloc.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dloc.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dloc.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dloc.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dloc.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dloc.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dloc.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dloc.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dloc.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dloc.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dloc.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dloc.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dloc.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dloc.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dloc.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dloc.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dloc.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dloc.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dloc.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dloc.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dloc.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dloc.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dloc.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dloc.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dloc.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dloc.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dloc.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dloc.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dloc.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dloc.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dloc.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dloc.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dloc.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dloc.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dloc.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dloc.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dloc.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dloc.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dloc.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dloc.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dloc.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;loc_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;locTree&quot; , &quot;'&quot; , &quot;); 
if(lname) { tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l&quot; , &quot;'&quot; , &quot;+id,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l-1&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;locArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dloc; 
  
  
  


	    
    
   
  
  
   資料類型:
   
BD-藍光光碟
CA-靜畫資料
DB-資料庫
DF-磁片
DO-電子書
EA-立體模型
EB-線上電子書
EJ-線上電子期刊
EP-電子期刊光碟
ERROR-有問題特藏
FA-磁帶
KT-多媒體組件
LA-地圖
LD-影碟
MP-MP3
NH-微縮單片
NR-微縮捲片
QA-地球儀
R-參考書
SL-幻燈片
VC-錄影帶
VD-VCD
BOX-書箱
ERM_DB-電子資料庫(ERM)
ERM_WS-網路資源(ERM)
ERM_EB-電子書(ERM)
ERM_EJ-電子期刊(ERM)
XL-X-ray
BOOK-圖書
AC-錄音帶
APP-附件
P-現期期刊
ac-ac
DD-影像光碟(DVD)
MD-行動設備
S-裝訂期刊
AD-唱片
CD-光碟
booklet-小冊子
KKtest-KK
YYtest2-YY2
YYtest4-YYYtest
0425-0425
TEST-TEST
TEST0425-TEST0425

   價格:
   
  
   
   館藏流通類別:
   
	
    
     	
B可借圖書
B電子資源
eee
M可借行動設備
P可借期刊
V可借視聽
www
不流通
書箱借閱30天

    
   
      	採購實價:
   		
   				
   				
   			
   			
   		
	
  
   條碼號類別:
   
     
      電子書
     
   
	  附件:
   
	
	  
  
  
    分類方法:
    

中文圖書分類法
美國國會圖書分類法
杜威分類法
美國國家醫學圖書館分類法
何日章中國圖書十進分類法

	 特藏:
	
測試2
測試3
PickListValues.15007340
PickListValues.15007339
PickListValues.15007338
PickListValues.15007337
商務企劃專案實務
PickListValues.15007336
測試0
通用特藏
禮物
測試4
電腦動畫視覺特效實務
電腦輔助繪圖
測試1
一般藏書
贈書
中英筆譯(二)
英語會話(二)
基礎寫作(二)
進階英語聽講練習(二)
進階實用英文寫作(二)
寫作入門(二)
專題研究方法
第二外語 (二) –日文
第二外語 (二) –西班牙文
第二外語聽講練習(二) –日文
第二外語聽講練習(二) –西班牙文
多媒體語言學習(二)
表演藝術(二)
語言測驗檢析(二)
閱讀與簡報(二)
實用英文寫作(二)
機械實務設計
測試5

	   
     
  
    分類號: 
    
	    
	    	
	    
    
	 分類法版本:
   
 
  
  
    作者號規則
    
      

科特號
何日章著者號
科特號(T2)

    
	館藏狀態:
	
	  
	  
	   
在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

	  
	 
	
  
  
    作者名字
    
     
       
    
    
	期刊架位號:
	
		
	  
  
  
   
  
    作者號
    
      
    
        
        


      
    
	   到館日期:
    
	  
  
    
  
  
   出版年:
   
	 卷號:
   
 
  

  
   索書號:
   
   		
   					
  
   					
			   			
			   		
			   		[AuthorNumber] +[ItemCollection] +[Year] +[ClassificationNumber] +[MaterialType] +[VolumeNumber] +[CopyNumber] 
   		
   
   複本號:   
	
    
    
     	
    
   

  
  
  
	於公共目錄顯示:
	
	館藏流通狀態:
	在架
	
   
  
  
   使用類型:
   
eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試
  
    電子資源連結:
   
  

  
   館員備註:
   (限1000字元)
   
   
  
  
   採購備註:
   
   備註欄:
   
 
   
  
   期刊備註:
   
   
   流通註記:
   
  
   
   
   其他號碼:
   
   
   
   
    最後盤點日期:
   
   
  
  
   
   
   財產編號:
   
   
   
   
   
   館藏經費:
   
   
    
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏經費2020 - 圖書 - -2001文學院-圖書費2020 - 圖書 - -2002商學院-圖書費2020 - 圖書 - -ah01安南醫院經費-2021 - 圖書 - -091文學院-2021 - 圖書 - -cwen1圖書費教育部補助2021 - 圖書 - -cwen101兒童書教育部補助2021 - 圖書 - -cwen102教科書教育部補助2021 - 圖書 - 20210825TEST20210825TEST-2021 - 圖書 - *new cost-2021 - 圖書 - -e2enew cost-2021 - 圖書 - GOGO-2021 - 視聽資料 - -cwen3DVD採購案學校補助2021 - 視聽資料 - -cwen31卡通影片學校補助2021 - 圖書 - -cwen311卡通電影學校補助2021 - 圖書 - NickBook尼克圖書經費人文社會學系2021 - 圖書 - -2021ah安南醫院經費學校款項2021 - 期刊 - J2021圖書館經費教育部補助2022 - 圖書 - JB01童書經費圖書館2023 - 圖書 - TEST234new cost-2023 - 圖書 - TEST111new cost-
  
  
window.ditemCostTree = new dTree(&quot; , &quot;'&quot; , &quot;window.ditemCostTree&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ditemCostTree.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏經費&quot; , &quot;'&quot; , &quot;); 
window.ditemCostTree.add(2502,0,&quot;2020 - 圖書 - -2001文學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -2001\u6587\u5B78\u9662-\u5716\u66F8\u8CBB&quot; , &quot;'&quot; , &quot;, 2502, true)&quot;); 
window.ditemCostTree.add(2525,0,&quot;2020 - 圖書 - -2002商學院-圖書費&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -2002\u5546\u5B78\u9662-\u5716\u66F8\u8CBB&quot; , &quot;'&quot; , &quot;, 2525, true)&quot;); 
window.ditemCostTree.add(2623,0,&quot;2020 - 圖書 - -ah01安南醫院經費-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2020 - \u5716\u66F8 - -ah01\u5B89\u5357\u91AB\u9662\u7D93\u8CBB-&quot; , &quot;'&quot; , &quot;, 2623, true)&quot;); 
window.ditemCostTree.add(3161,0,&quot;2021 - 圖書 - -091文學院-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -091\u6587\u5B78\u9662-&quot; , &quot;'&quot; , &quot;, 3161, true)&quot;); 
window.ditemCostTree.add(2661,0,&quot;2021 - 圖書 - -cwen1圖書費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen1\u5716\u66F8\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2661, true)&quot;); 
window.ditemCostTree.add(2681,2661,&quot;2021 - 圖書 - -cwen101兒童書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen101\u5152\u7AE5\u66F8\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2681, true)&quot;); 
window.ditemCostTree.add(2682,2661,&quot;2021 - 圖書 - -cwen102教科書教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen102\u6559\u79D1\u66F8\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2682, true)&quot;); 
window.ditemCostTree.add(3441,0,&quot;2021 - 圖書 - 20210825TEST20210825TEST-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - 20210825TEST20210825TEST-&quot; , &quot;'&quot; , &quot;, 3441, true)&quot;); 
window.ditemCostTree.add(3421,0,&quot;2021 - 圖書 - *new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - *new cost-&quot; , &quot;'&quot; , &quot;, 3421, true)&quot;); 
window.ditemCostTree.add(3461,0,&quot;2021 - 圖書 - -e2enew cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -e2enew cost-&quot; , &quot;'&quot; , &quot;, 3461, true)&quot;); 
window.ditemCostTree.add(3081,0,&quot;2021 - 圖書 - GOGO-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - GOGO-&quot; , &quot;'&quot; , &quot;, 3081, true)&quot;); 
window.ditemCostTree.add(2683,0,&quot;2021 - 視聽資料 - -cwen3DVD採購案學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u8996\u807D\u8CC7\u6599 - -cwen3DVD\u63A1\u8CFC\u6848\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2683, true)&quot;); 
window.ditemCostTree.add(2721,2683,&quot;2021 - 視聽資料 - -cwen31卡通影片學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u8996\u807D\u8CC7\u6599 - -cwen31\u5361\u901A\u5F71\u7247\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2721, true)&quot;); 
window.ditemCostTree.add(2781,2721,&quot;2021 - 圖書 - -cwen311卡通電影學校補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -cwen311\u5361\u901A\u96FB\u5F71\u5B78\u6821\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 2781, true)&quot;); 
window.ditemCostTree.add(3261,0,&quot;2021 - 圖書 - NickBook尼克圖書經費人文社會學系&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - NickBook\u5C3C\u514B\u5716\u66F8\u7D93\u8CBB\u4EBA\u6587\u793E\u6703\u5B78\u7CFB&quot; , &quot;'&quot; , &quot;, 3261, true)&quot;); 
window.ditemCostTree.add(2801,0,&quot;2021 - 圖書 - -2021ah安南醫院經費學校款項&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u5716\u66F8 - -2021ah\u5B89\u5357\u91AB\u9662\u7D93\u8CBB\u5B78\u6821\u6B3E\u9805&quot; , &quot;'&quot; , &quot;, 2801, true)&quot;); 
window.ditemCostTree.add(3221,0,&quot;2021 - 期刊 - J2021圖書館經費教育部補助&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2021 - \u671F\u520A - J2021\u5716\u66F8\u9928\u7D93\u8CBB\u6559\u80B2\u90E8\u88DC\u52A9&quot; , &quot;'&quot; , &quot;, 3221, true)&quot;); 
window.ditemCostTree.add(3121,0,&quot;2022 - 圖書 - JB01童書經費圖書館&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2022 - \u5716\u66F8 - JB01\u7AE5\u66F8\u7D93\u8CBB\u5716\u66F8\u9928&quot; , &quot;'&quot; , &quot;, 3121, true)&quot;); 
window.ditemCostTree.add(3641,0,&quot;2023 - 圖書 - TEST234new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2023 - \u5716\u66F8 - TEST234new cost-&quot; , &quot;'&quot; , &quot;, 3641, true)&quot;); 
window.ditemCostTree.add(3601,0,&quot;2023 - 圖書 - TEST111new cost-&quot;, &quot;javascript:window.ditemCostTree.selectElement(&quot; , &quot;'&quot; , &quot;2023 - \u5716\u66F8 - TEST111new cost-&quot; , &quot;'&quot; , &quot;, 3601, true)&quot;); 
window.ditemCostTree.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;itemCostTree_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_0&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;itemCostTreeTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;itemCostTreeArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ditemCostTree; 
  
  
  


   
   
  
   
   	
   	 	新增日期:
   	 	
   	
  		2021/03/23 15:32:03
   	
   	
   	
   	
   
 
 
 
 
 
 
 
 



 



	
     
       
       
         修改/存檔
       
     
     
     取消
     
  





  
	館藏數量:
	 
  
  
	產生方式:
	
	  
	   
	    自動產生
	    手動輸入
	   
	  
	
  

   

 
  
    產生
  
 









 
    
 
  
    Author nr
  
  
 
 
  
 

 


	clearCallNumberItem();
	clearPickListValue();
	setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AuthorNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ItemCollection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Year&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ClassificationNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MaterialType&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VolumeNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;  &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); setCallNumberItem(&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CopyNumber&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;); 
	var isOmitIfmaterialTypeBook = true
	setCallNumber(getCallNumberItem(), isOmitIfmaterialTypeBook);
	setPickListValue(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DF&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DO&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;6&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;7&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EJ&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;8&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;9&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERROR&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KT&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;12&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;13&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;14&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;15&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NH&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;16&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NR&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;17&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;18&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;R&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;19&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SL&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VC&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;21&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;22&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BOX&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;23&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_DB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;24&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_WS&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_EB&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;26&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ERM_EJ&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;27&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;XL&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;28&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BOOK&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;29&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AC&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;30&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;APP&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;31&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;P&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;32&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ac&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;33&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;34&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;35&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;S&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;36&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;37&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CD&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;38&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;booklet&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;39&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KKtest&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;40&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;YYtest2&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;41&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;YYtest4&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;42&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;0425&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;43&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TEST&quot; , &quot;'&quot; , &quot;); setPickListValue(&quot; , &quot;'&quot; , &quot;44&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TEST0425&quot; , &quot;'&quot; , &quot;); 
	doCallNumber();


 




	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;editForm&quot;);
calendar_DatePicker = new Calendar(1616428800000);
	
calendar_DatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_DatePicker.onchange = function() {
  var field = tapestry.byId(&quot;editForm&quot;).DatePicker;
  var value = calendar_DatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }

closeDialogComponent(&quot; , &quot;'&quot; , &quot;AuthornrDialog&quot; , &quot;'&quot; , &quot;);
try {
     document.getElementById(&quot;generateArea&quot;).style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    }catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;CloseReminderDialog&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);});
// -->




/html[1]&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六 12345678910111213141516171819202122232425262728293031          15 三月, 2024Clear&quot;))]</value>
      <webElementGuid>09b9ab12-24f6-4078-bfff-5f7ea55020af</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
