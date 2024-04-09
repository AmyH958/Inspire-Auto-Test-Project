<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_DetailsdjConfig  baseRelativePathinspi_19211e</name>
   <tag></tag>
   <elementGuidId>462389ca-0709-4e13-a4c9-fda0374103d4</elementGuidId>
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
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>15737541-9594-4561-bdcb-34f5665053c6</webElementGuid>
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

var calendar_lockedDatetimeDatePicker;
var calendar_equEndDateDatePicker;
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

 

	

	
		
		  
		    
	
		讀者資料維護
		
		
			
		        讀者基本資料
			
		
	

		  
		  
		  
		    
			
			
		  
		


	
	     


 


 
  
   
    
    
    一般細節
   
  
  
 
 
  
	
		
	
	
	
	
	
		$(document).keypress(function(e) {
			if (e.which == 13) {
				return false;
			}
		});
	
	
		(function($) {
			//日期選擇器
			function initDatePicker() {
		        var contextPath = trim($('#contextPath').text());
		        $('.DatePickerID').datepick({
		            dateFormat: 'yy/mm/dd',
		            showOn: 'button',
		            buttonImageOnly: true,
		            buttonImage: contextPath + '/js/datepick/DatePickerIcon.png',
		            yearRange: 'c-100:c+50'
		        });
		    }
		    // 在文檔準備好時初始化日期選擇器
		    $(document).ready(function() {
		        initDatePicker();
		    });
		 	// 使用MutationObserver監聽文檔內容的變化
		    var observer = new MutationObserver(function(mutations) {
		        mutations.forEach(function(mutation) {
		            if (mutation.type === 'childList') {
		                // 文檔內容發生變化時重新初始化日期選擇器
		                initDatePicker();
		            }
		        });
		    });
		    // 監聽整個文檔的變化
		    observer.observe(document, { childList: true, subtree: true });
		})(jQuery);
	
	
		(function($) {
			$(document).ready(function() {
				$(&quot;#dateToBeCheckedflag&quot;).hide();
				if ($(&quot;#isLanguage&quot;).html() == &quot;1&quot;) {
					$(&quot;#dateToBeCheckedflag&quot;).show();
				}
			});
		})(jQuery);
	
	

#loading-overlay { position: absolute; z-index: 2147483647; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: -500; background-color: transparent; opacity: 0.7; }

	




















		 
	function blockScreen_0(e){
		var iDiv = window.document.createElement('div');
		iDiv.id = 'loading-overlay'; 
		var blockDiv = document.getElementById('blockDiv_0');
		if (blockDiv!=null) blockDiv.appendChild(iDiv);
	} 
	document.addEventListener('DOMContentLoaded', function() {
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButton&quot;), &quot;onclick&quot;, blockScreen_0);
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButtonWithQuest&quot;), &quot;onclick&quot;, blockScreen_0);
	}, false);





//&lt;![CDATA[

	 function deleteblock_0(){
		var parent = document.getElementById('blockDiv_0');
		var child = document.getElementById('loading-overlay');		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock_0();
 
//]]&gt;




	
		輸入欄位驗証錯誤提示行動電話電話號碼格式錯誤 
		1
		
			/inspireapp/
		
		
			   
		
		
		
		
			
						
				
					*姓名:
					

					暱稱:
					
				
				
					*帳號:
					
							
						 
							   1234  帳號修改
							
						

					* 身分證號:
					
				
				
					行動電話:
					







					讀者狀態:
					
有效讀者
離職離校
微波通訊
MATLAB程式設計
無線行動網路架構
電力電子學
硬體描述語言設計
電路板設計實務
PHP程式設計
通訊實習
手持式裝置設計與應用
進階數學
半導體元件及量測實務
科技英文
創意與專利1

				
				
					單位所系:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
所系單位_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree('window.ddepartment', 'messages', '/inspireapp/images/'); 
window.ddepartment.add(0,-1,'所系單位'); 
window.ddepartment.add(441,0,&quot;_New Item&quot;, &quot;javascript:window.ddepartment.selectElement('_New Item', 441, true)&quot;); 
window.ddepartment.add(461,0,&quot;_New Item1234&quot;, &quot;javascript:window.ddepartment.selectElement('_New Item1234', 461, true)&quot;); 
window.ddepartment.add(121,0,&quot;\u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddepartment.selectElement('\\u795E\\u8CC7\\u5716\\u66F8\\u9928', 121, true)&quot;); 
window.ddepartment.add(141,121,&quot;\u884C\u653F\u55AE\u4F4D\u5176\u4ED6\u5176\u4ED6\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement('\\u884C\\u653F\\u55AE\\u4F4D\\u5176\\u4ED6\\u5176\\u4ED6\\u5176\\u4ED6', 141, true)&quot;); 
window.ddepartment.add(145,141,&quot;\u5B78\u6821\u884C\u653F&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B78\\u6821\\u884C\\u653F', 145, true)&quot;); 
window.ddepartment.add(167,145,&quot;\u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddepartment.selectElement('\\u53F0\\u4E2D\\u7E3D\\u9928', 167, true)&quot;); 
window.ddepartment.add(168,145,&quot;\u5317\u6E2F\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5317\\u6E2F\\u5716\\u66F8\\u5206\\u9928', 168, true)&quot;); 
window.ddepartment.add(122,145,&quot;\u5B89\u5357\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B89\\u5357\\u5716\\u66F8\\u5206\\u9928', 122, true)&quot;); 
window.ddepartment.add(123,145,&quot;\u6C34\u6E73\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6C34\\u6E73\\u5716\\u66F8\\u5206\\u9928', 123, true)&quot;); 
window.ddepartment.add(124,145,&quot;\u5B78\u6821\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B78\\u6821\\u884C\\u653F\\u55AE\\u4F4D', 124, true)&quot;); 
window.ddepartment.add(125,145,&quot;\u5317\u6E2F\u5206\u90E8\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5317\\u6E2F\\u5206\\u90E8\\u884C\\u653F\\u55AE\\u4F4D', 125, true)&quot;); 
window.ddepartment.add(126,145,&quot;\u5B78\u6821\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B78\\u6821\\u7814\\u7A76\\u4E2D\\u5FC3', 126, true)&quot;); 
window.ddepartment.add(142,121,&quot;\u5B78\u8853\u55AE\u4F4D\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B78\\u8853\\u55AE\\u4F4D\\u5176\\u4ED6', 142, true)&quot;); 
window.ddepartment.add(146,142,&quot;\u91AB\u5B78\u9662\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u9662\\u5176\\u4ED6', 146, true)&quot;); 
window.ddepartment.add(127,146,&quot;\u91AB\u5B78\u7CFB\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u7CFB\\u5176\\u4ED6', 127, true)&quot;); 
window.ddepartment.add(401,127,&quot;\u91AB\u5B78\u7CFB\u4E00\u5E74A\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u7CFB\\u4E00\\u5E74A\\u73ED', 401, true)&quot;); 
window.ddepartment.add(128,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB', 128, true)&quot;); 
window.ddepartment.add(129,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB', 129, true)&quot;); 
window.ddepartment.add(130,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED', 130, true)&quot;); 
window.ddepartment.add(131,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 131, true)&quot;); 
window.ddepartment.add(132,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 132, true)&quot;); 
window.ddepartment.add(133,146,&quot;\u570B\u969B\u751F\u7269\u91AB\u5B78\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u570B\\u969B\\u751F\\u7269\\u91AB\\u5B78\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 133, true)&quot;); 
window.ddepartment.add(134,146,&quot;\u81E8\u5E8A\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u81E8\\u5E8A\\u91AB\\u5B78\\u7814\\u7A76\\u6240', 134, true)&quot;); 
window.ddepartment.add(135,146,&quot;\u57FA\u790E\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u57FA\\u790E\\u91AB\\u5B78\\u7814\\u7A76\\u6240', 135, true)&quot;); 
window.ddepartment.add(136,146,&quot;\u514D\u75AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u514D\\u75AB\\u5B78\\u7814\\u7A76\\u6240', 136, true)&quot;); 
window.ddepartment.add(137,146,&quot;\u764C\u75C7\u751F\u7269\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u764C\\u75C7\\u751F\\u7269\\u5B78\\u7814\\u7A76\\u6240', 137, true)&quot;); 
window.ddepartment.add(138,146,&quot;\u795E\u7D93\u79D1\u5B78\u8207\u8A8D\u77E5\u79D1\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u795E\\u7D93\\u79D1\\u5B78\\u8207\\u8A8D\\u77E5\\u79D1\\u5B78\\u7814\\u7A76\\u6240', 138, true)&quot;); 
window.ddepartment.add(139,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED', 139, true)&quot;); 
window.ddepartment.add(169,146,&quot;\u764C\u75C7\u751F\u7269\u8207\u85E5\u7269\u7814\u767C\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u764C\\u75C7\\u751F\\u7269\\u8207\\u85E5\\u7269\\u7814\\u767C\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 169, true)&quot;); 
window.ddepartment.add(170,146,&quot;\u8001\u5316\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8001\\u5316\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 170, true)&quot;); 
window.ddepartment.add(171,146,&quot;\u8F49\u8B6F\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8F49\\u8B6F\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 171, true)&quot;); 
window.ddepartment.add(321,146,&quot;\u751F\u91AB\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u91AB\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 321, true)&quot;); 
window.ddepartment.add(147,142,&quot;\u4E2D\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u9662', 147, true)&quot;); 
window.ddepartment.add(421,147,&quot;New Item&quot;, &quot;javascript:window.ddepartment.selectElement('New Item', 421, true)&quot;); 
window.ddepartment.add(172,147,&quot;\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u7CFB', 172, true)&quot;); 
window.ddepartment.add(173,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u7532\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u7CFB\\u7532\\u7D44', 173, true)&quot;); 
window.ddepartment.add(174,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u4E59\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u7CFB\\u4E59\\u7D44', 174, true)&quot;); 
window.ddepartment.add(175,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB', 175, true)&quot;); 
window.ddepartment.add(176,147,&quot;\u5B78\u58EB\u5F8C\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B78\\u58EB\\u5F8C\\u4E2D\\u91AB\\u5B78\\u7CFB', 176, true)&quot;); 
window.ddepartment.add(177,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 177, true)&quot;); 
window.ddepartment.add(178,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED', 178, true)&quot;); 
window.ddepartment.add(140,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED', 140, true)&quot;); 
window.ddepartment.add(181,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 181, true)&quot;); 
window.ddepartment.add(182,147,&quot;\u570B\u969B\u91DD\u7078\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u570B\\u969B\\u91DD\\u7078\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 182, true)&quot;); 
window.ddepartment.add(183,147,&quot;\u4E2D\u7378\u91AB\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u7378\\u91AB\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 183, true)&quot;); 
window.ddepartment.add(184,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 184, true)&quot;); 
window.ddepartment.add(185,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED', 185, true)&quot;); 
window.ddepartment.add(186,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED', 186, true)&quot;); 
window.ddepartment.add(187,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 187, true)&quot;); 
window.ddepartment.add(148,142,&quot;\u85E5\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u5B78\\u9662', 148, true)&quot;); 
window.ddepartment.add(179,148,&quot;\u85E5\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u5B78\\u7CFB', 179, true)&quot;); 
window.ddepartment.add(180,148,&quot;\u85E5\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 180, true)&quot;); 
window.ddepartment.add(201,148,&quot;\u85E5\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 201, true)&quot;); 
window.ddepartment.add(202,148,&quot;\u751F\u6280\u88FD\u85E5\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u6280\\u88FD\\u85E5\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 202, true)&quot;); 
window.ddepartment.add(149,142,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662', 149, true)&quot;); 
window.ddepartment.add(203,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB', 203, true)&quot;); 
window.ddepartment.add(204,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB', 204, true)&quot;); 
window.ddepartment.add(205,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB', 205, true)&quot;); 
window.ddepartment.add(206,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662\u5927\u4E00\u4E0D\u5206\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662\\u5927\\u4E00\\u4E0D\\u5206\\u7CFB', 206, true)&quot;); 
window.ddepartment.add(207,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB', 207, true)&quot;); 
window.ddepartment.add(208,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 208, true)&quot;); 
window.ddepartment.add(209,149,&quot;\u516C\u5171\u885B\u751F\u570B\u969B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u570B\\u969B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 209, true)&quot;); 
window.ddepartment.add(210,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 210, true)&quot;); 
window.ddepartment.add(211,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED', 211, true)&quot;); 
window.ddepartment.add(212,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 212, true)&quot;); 
window.ddepartment.add(213,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED', 213, true)&quot;); 
window.ddepartment.add(214,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 214, true)&quot;); 
window.ddepartment.add(215,149,&quot;\u751F\u7269\u7D71\u8A08\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u7D71\\u8A08\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED', 215, true)&quot;); 
window.ddepartment.add(216,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 216, true)&quot;); 
window.ddepartment.add(381,149,&quot;\u55AE\u4F4D\u7CFB\u6240\u532F\u5165&quot;, &quot;javascript:window.ddepartment.selectElement('\\u55AE\\u4F4D\\u7CFB\\u6240\\u532F\\u5165', 381, true)&quot;); 
window.ddepartment.add(191,149,&quot;\u91AB\u5B78\u5DE5\u7A0B\u8207\u5FA9\u5065\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u5DE5\\u7A0B\\u8207\\u5FA9\\u5065\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 191, true)&quot;); 
window.ddepartment.add(245,149,&quot;\u751F\u7269\u91AB\u5B78\u5DE5\u7A0B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u91AB\\u5B78\\u5DE5\\u7A0B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 245, true)&quot;); 
window.ddepartment.add(150,142,&quot;\u5065\u5EB7\u7167\u8B77\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5065\\u5EB7\\u7167\\u8B77\\u5B78\\u9662', 150, true)&quot;); 
window.ddepartment.add(217,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB', 217, true)&quot;); 
window.ddepartment.add(218,150,&quot;\u8B77\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8B77\\u7406\\u5B78\\u7CFB', 218, true)&quot;); 
window.ddepartment.add(219,150,&quot;\u904B\u52D5\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u904B\\u52D5\\u91AB\\u5B78\\u7CFB', 219, true)&quot;); 
window.ddepartment.add(220,150,&quot;\u53E3\u8154\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u53E3\\u8154\\u885B\\u751F\\u5B78\\u7CFB', 220, true)&quot;); 
window.ddepartment.add(221,150,&quot;\u4E8C\u5E74\u5236\u8B77\u7406\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E8C\\u5E74\\u5236\\u8B77\\u7406\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED', 221, true)&quot;); 
window.ddepartment.add(188,150,&quot;\u4E8C\u5E74\u5236\u547C\u5438\u6CBB\u7642\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E8C\\u5E74\\u5236\\u547C\\u5438\\u6CBB\\u7642\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED', 188, true)&quot;); 
window.ddepartment.add(189,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB\u5FA9\u5065\u79D1\u5B78\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB\\u5FA9\\u5065\\u79D1\\u5B78\\u78A9\\u58EB\\u73ED', 189, true)&quot;); 
window.ddepartment.add(190,150,&quot;\u8B77\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8B77\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 190, true)&quot;); 
window.ddepartment.add(361,150,&quot;\u8B77\u7406\u5B78\u7CFB\u8DE8\u9818\u57DF\u9577\u671F\u7167\u8B77\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8B77\\u7406\\u5B78\\u7CFB\\u8DE8\\u9818\\u57DF\\u9577\\u671F\\u7167\\u8B77\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED', 361, true)&quot;); 
window.ddepartment.add(192,150,&quot;\u5065\u5EB7\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5065\\u5EB7\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 192, true)&quot;); 
window.ddepartment.add(151,142,&quot;\u751F\u6280\u88FD\u85E5\u66A8\u98DF\u54C1\u79D1\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u6280\\u88FD\\u85E5\\u66A8\\u98DF\\u54C1\\u79D1\\u5B78\\u9662', 151, true)&quot;); 
window.ddepartment.add(193,151,&quot;\u71DF\u990A\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u71DF\\u990A\\u5B78\\u7CFB', 193, true)&quot;); 
window.ddepartment.add(194,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB', 194, true)&quot;); 
window.ddepartment.add(195,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB', 195, true)&quot;); 
window.ddepartment.add(196,151,&quot;\u71DF\u990A\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u71DF\\u990A\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 196, true)&quot;); 
window.ddepartment.add(197,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 197, true)&quot;); 
window.ddepartment.add(198,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 198, true)&quot;); 
window.ddepartment.add(199,151,&quot;\u88FD\u85E5\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u88FD\\u85E5\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 199, true)&quot;); 
window.ddepartment.add(200,151,&quot;\u98DF\u54C1\u66A8\u85E5\u7269\u5B89\u5168\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u98DF\\u54C1\\u66A8\\u85E5\\u7269\\u5B89\\u5168\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 200, true)&quot;); 
window.ddepartment.add(241,151,&quot;\u71DF\u990A\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u71DF\\u990A\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 241, true)&quot;); 
window.ddepartment.add(242,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 242, true)&quot;); 
window.ddepartment.add(243,151,&quot;\u65B0\u85E5\u958B\u767C\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u65B0\\u85E5\\u958B\\u767C\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED', 243, true)&quot;); 
window.ddepartment.add(244,151,&quot;\u751F\u7269\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u751F\\u7269\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 244, true)&quot;); 
window.ddepartment.add(152,142,&quot;\u4EBA\u6587\u8207\u79D1\u6280\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4EBA\\u6587\\u8207\\u79D1\\u6280\\u5B78\\u9662', 152, true)&quot;); 
window.ddepartment.add(322,152,&quot;\u79D1\u6280\u6CD5\u5F8B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement('\\u79D1\\u6280\\u6CD5\\u5F8B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B\\u5176\\u4ED6', 322, true)&quot;); 
window.ddepartment.add(362,152,&quot;\u79D1\u6280\u7BA1\u7406\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement('\\u79D1\\u6280\\u7BA1\\u7406\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B', 362, true)&quot;); 
window.ddepartment.add(153,142,&quot;\u7259\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u5B78\\u9662', 153, true)&quot;); 
window.ddepartment.add(246,153,&quot;\u7259\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u5B78\\u7CFB', 246, true)&quot;); 
window.ddepartment.add(247,153,&quot;\u7259\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED', 247, true)&quot;); 
window.ddepartment.add(363,153,&quot;\u7259\u91AB\u5B78\u7CFB\u53E3\u8154\u91AB\u5B78\u7522\u696D\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u5B78\\u7CFB\\u53E3\\u8154\\u91AB\\u5B78\\u7522\\u696D\\u78A9\\u58EB\\u73ED', 363, true)&quot;); 
window.ddepartment.add(323,153,&quot;\u7259\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED', 323, true)&quot;); 
window.ddepartment.add(154,142,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3', 154, true)&quot;); 
window.ddepartment.add(248,154,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3', 248, true)&quot;); 
window.ddepartment.add(143,121,&quot;\u9644\u8A2D\u6A5F\u69CB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u8A2D\\u6A5F\\u69CB', 143, true)&quot;); 
window.ddepartment.add(222,143,&quot;\u4E2D\u570B\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u570B\\u9644\\u91AB', 222, true)&quot;); 
window.ddepartment.add(223,222,&quot;\u9644\u91AB\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB\\u7814\\u7A76\\u4E2D\\u5FC3', 223, true)&quot;); 
window.ddepartment.add(224,222,&quot;\u5167\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5167\\u79D1\\u90E8', 224, true)&quot;); 
window.ddepartment.add(225,222,&quot;\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5916\\u79D1\\u90E8', 225, true)&quot;); 
window.ddepartment.add(226,222,&quot;\u795E\u7D93\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u795E\\u7D93\\u5916\\u79D1\\u90E8', 226, true)&quot;); 
window.ddepartment.add(249,222,&quot;\u9AA8\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9AA8\\u79D1\\u90E8', 249, true)&quot;); 
window.ddepartment.add(250,222,&quot;\u6CCC\u5C3F\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6CCC\\u5C3F\\u90E8', 250, true)&quot;); 
window.ddepartment.add(251,222,&quot;\u5A66\u7522\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5A66\\u7522\\u90E8', 251, true)&quot;); 
window.ddepartment.add(227,222,&quot;\u795E\u7D93\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u795E\\u7D93\\u90E8', 227, true)&quot;); 
window.ddepartment.add(228,222,&quot;\u8033\u9F3B\u5589\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8033\\u9F3B\\u5589\\u90E8', 228, true)&quot;); 
window.ddepartment.add(229,222,&quot;\u76AE\u819A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement('\\u76AE\\u819A\\u79D1', 229, true)&quot;); 
window.ddepartment.add(230,222,&quot;\u7259\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7259\\u91AB\\u90E8', 230, true)&quot;); 
window.ddepartment.add(231,222,&quot;\u7CBE\u795E\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u7CBE\\u795E\\u91AB\\u5B78\\u90E8', 231, true)&quot;); 
window.ddepartment.add(232,222,&quot;\u5FA9\u5065\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5FA9\\u5065\\u90E8', 232, true)&quot;); 
window.ddepartment.add(233,222,&quot;\u9EBB\u9189\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9EBB\\u9189\\u90E8', 233, true)&quot;); 
window.ddepartment.add(235,222,&quot;\u81E8\u5E8A\u71DF\u990A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement('\\u81E8\\u5E8A\\u71DF\\u990A\\u79D1', 235, true)&quot;); 
window.ddepartment.add(234,222,&quot;\u4E2D\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u91AB\\u90E8', 234, true)&quot;); 
window.ddepartment.add(252,222,&quot;\u4E2D\u570B\u9644\u91AB\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u570B\\u9644\\u91AB\\u884C\\u653F\\u55AE\\u4F4D', 252, true)&quot;); 
window.ddepartment.add(253,222,&quot;\u793E\u6703\u5DE5\u4F5C\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement('\\u793E\\u6703\\u5DE5\\u4F5C\\u5BA4', 253, true)&quot;); 
window.ddepartment.add(254,222,&quot;\u773C\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u773C\\u79D1\\u90E8', 254, true)&quot;); 
window.ddepartment.add(255,222,&quot;\u5152\u7AE5\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5152\\u7AE5\\u91AB\\u9662', 255, true)&quot;); 
window.ddepartment.add(256,222,&quot;\u75C5\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u75C5\\u7406\\u90E8', 256, true)&quot;); 
window.ddepartment.add(257,222,&quot;\u57FA\u56E0\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u57FA\\u56E0\\u91AB\\u5B78\\u90E8', 257, true)&quot;); 
window.ddepartment.add(258,222,&quot;\u9810\u9632\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9810\\u9632\\u91AB\\u5B78\\u4E2D\\u5FC3', 258, true)&quot;); 
window.ddepartment.add(259,222,&quot;\u91AB\u5B78\u7814\u7A76\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u7814\\u7A76\\u90E8', 259, true)&quot;); 
window.ddepartment.add(260,222,&quot;\u6559\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6559\\u5B78\\u90E8', 260, true)&quot;); 
window.ddepartment.add(261,222,&quot;\u6025\u75C7\u66A8\u5916\u50B7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6025\\u75C7\\u66A8\\u5916\\u50B7\\u4E2D\\u5FC3', 261, true)&quot;); 
window.ddepartment.add(262,222,&quot;\u8B77\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8B77\\u7406\\u90E8', 262, true)&quot;); 
window.ddepartment.add(263,222,&quot;\u85E5\u5291\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u85E5\\u5291\\u90E8', 263, true)&quot;); 
window.ddepartment.add(264,222,&quot;\u91AB\u5B78\u5F71\u50CF\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u5B78\\u5F71\\u50CF\\u90E8', 264, true)&quot;); 
window.ddepartment.add(265,222,&quot;\u6AA2\u9A57\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6AA2\\u9A57\\u91AB\\u5B78\\u90E8', 265, true)&quot;); 
window.ddepartment.add(266,222,&quot;\u6838\u5B50\u91AB\u5B78\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6838\\u5B50\\u91AB\\u5B78\\u79D1', 266, true)&quot;); 
window.ddepartment.add(267,222,&quot;\u795E\u7D93\u7CBE\u795E\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u795E\\u7D93\\u7CBE\\u795E\\u91AB\\u5B78\\u4E2D\\u5FC3', 267, true)&quot;); 
window.ddepartment.add(268,222,&quot;\u91AB\u7642\u54C1\u8CEA\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement('\\u91AB\\u7642\\u54C1\\u8CEA\\u90E8', 268, true)&quot;); 
window.ddepartment.add(269,222,&quot;\u764C\u75C7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u764C\\u75C7\\u4E2D\\u5FC3', 269, true)&quot;); 
window.ddepartment.add(155,143,&quot;\u9644\u91AB-\u5317\u6E2F\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u5317\\u6E2F\\u9644\\u91AB', 155, true)&quot;); 
window.ddepartment.add(270,155,&quot;\u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662', 270, true)&quot;); 
window.ddepartment.add(156,143,&quot;\u9644\u91AB-\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u8C50\\u539F\\u5206\\u9662', 156, true)&quot;); 
window.ddepartment.add(271,156,&quot;\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8C50\\u539F\\u5206\\u9662', 271, true)&quot;); 
window.ddepartment.add(157,143,&quot;\u9644\u91AB-\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4', 157, true)&quot;); 
window.ddepartment.add(272,157,&quot;\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4', 272, true)&quot;); 
window.ddepartment.add(158,143,&quot;\u9644\u91AB-\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662', 158, true)&quot;); 
window.ddepartment.add(273,158,&quot;\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662', 273, true)&quot;); 
window.ddepartment.add(159,143,&quot;\u9644\u91AB-\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u53F0\\u5317\\u5206\\u9662', 159, true)&quot;); 
window.ddepartment.add(274,159,&quot;\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u53F0\\u5317\\u5206\\u9662', 274, true)&quot;); 
window.ddepartment.add(160,143,&quot;\u9644\u91AB-\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662', 160, true)&quot;); 
window.ddepartment.add(275,160,&quot;\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662', 275, true)&quot;); 
window.ddepartment.add(301,143,&quot;\u9644\u91AB-\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240', 301, true)&quot;); 
window.ddepartment.add(302,301,&quot;\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240', 302, true)&quot;); 
window.ddepartment.add(161,143,&quot;\u9644\u91AB-\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u8349\\u5C6F\\u5206\\u9662', 161, true)&quot;); 
window.ddepartment.add(276,161,&quot;\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u8349\\u5C6F\\u5206\\u9662', 276, true)&quot;); 
window.ddepartment.add(162,143,&quot;\u9644\u91AB-\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662', 162, true)&quot;); 
window.ddepartment.add(277,162,&quot;\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662', 277, true)&quot;); 
window.ddepartment.add(163,143,&quot;\u9644\u91AB-\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3', 163, true)&quot;); 
window.ddepartment.add(278,163,&quot;\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3', 278, true)&quot;); 
window.ddepartment.add(164,143,&quot;\u9644\u91AB-\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9644\\u91AB-\\u5B89\\u5357\\u91AB\\u9662', 164, true)&quot;); 
window.ddepartment.add(279,164,&quot;\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement('\\u5B89\\u5357\\u91AB\\u9662', 279, true)&quot;); 
window.ddepartment.add(144,121,&quot;\u6821\u5916\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6821\\u5916\\u55AE\\u4F4D', 144, true)&quot;); 
window.ddepartment.add(165,144,&quot;\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement('\\u9928\\u969B\\u5408\\u4F5C', 165, true)&quot;); 
window.ddepartment.add(236,165,&quot;NDDS\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement('NDDS\\u9928\\u969B\\u5408\\u4F5C', 236, true)&quot;); 
window.ddepartment.add(237,165,&quot;\u4E92\u501F\u5354\u8B70\u806F\u76DF&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E92\\u501F\\u5354\\u8B70\\u806F\\u76DF', 237, true)&quot;); 
window.ddepartment.add(238,165,&quot;\u4E2D\u76DF-\u5927\u8449\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5927\\u8449\\u5927\\u5B78', 238, true)&quot;); 
window.ddepartment.add(239,165,&quot;\u4E2D\u76DF-\u4E2D\u5C71\u91AB\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u4E2D\\u5C71\\u91AB\\u5927', 239, true)&quot;); 
window.ddepartment.add(240,165,&quot;\u4E2D\u76DF-\u4E2D\u81FA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u4E2D\\u81FA\\u79D1\\u5927', 240, true)&quot;); 
window.ddepartment.add(281,165,&quot;\u4E2D\u76DF-\u4E2D\u8208\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u4E2D\\u8208\\u5927\\u5B78', 281, true)&quot;); 
window.ddepartment.add(282,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u6559\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u6559\\u5927', 282, true)&quot;); 
window.ddepartment.add(283,165,&quot;\u4E2D\u76DF-\u5F18\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5F18\\u5149\\u79D1\\u5927', 283, true)&quot;); 
window.ddepartment.add(284,165,&quot;\u4E2D\u76DF-\u4E9E\u6D32\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u4E9E\\u6D32\\u5927\\u5B78', 284, true)&quot;); 
window.ddepartment.add(285,165,&quot;\u4E2D\u76DF-\u6771\u6D77\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u6771\\u6D77\\u5927\\u5B78', 285, true)&quot;); 
window.ddepartment.add(286,165,&quot;\u4E2D\u76DF-\u5EFA\u570B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5EFA\\u570B\\u79D1\\u5927', 286, true)&quot;); 
window.ddepartment.add(287,165,&quot;\u4E2D\u76DF-\u66A8\u5357\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u66A8\\u5357\\u5927\\u5B78', 287, true)&quot;); 
window.ddepartment.add(288,165,&quot;\u4E2D\u76DF-\u9022\u7532\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u9022\\u7532\\u5927\\u5B78', 288, true)&quot;); 
window.ddepartment.add(289,165,&quot;\u4E2D\u76DF-\u671D\u967D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u671D\\u967D\\u79D1\\u5927', 289, true)&quot;); 
window.ddepartment.add(290,165,&quot;\u4E2D\u76DF-\u52E4\u76CA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u52E4\\u76CA\\u79D1\\u5927', 290, true)&quot;); 
window.ddepartment.add(291,165,&quot;\u4E2D\u76DF-\u5F70\u5316\u5E2B\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5F70\\u5316\\u5E2B\\u5927', 291, true)&quot;); 
window.ddepartment.add(292,165,&quot;\u4E2D\u76DF-\u975C\u5B9C\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u975C\\u5B9C\\u5927\\u5B78', 292, true)&quot;); 
window.ddepartment.add(293,165,&quot;\u4E2D\u76DF-\u5DBA\u6771\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5DBA\\u6771\\u79D1\\u5927', 293, true)&quot;); 
window.ddepartment.add(294,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u79D1\\u5927', 294, true)&quot;); 
window.ddepartment.add(295,165,&quot;\u4E2D\u76DF-\u806F\u5408\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u806F\\u5408\\u5927\\u5B78', 295, true)&quot;); 
window.ddepartment.add(296,165,&quot;\u4E2D\u76DF-\u660E\u9053\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u660E\\u9053\\u5927\\u5B78', 296, true)&quot;); 
window.ddepartment.add(297,165,&quot;\u4E2D\u76DF-\u5357\u958B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u5357\\u958B\\u79D1\\u5927', 297, true)&quot;); 
window.ddepartment.add(298,165,&quot;\u4E2D\u76DF-\u4FEE\u5E73\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u4FEE\\u5E73\\u79D1\\u5927', 298, true)&quot;); 
window.ddepartment.add(299,165,&quot;\u4E2D\u76DF-\u80B2\u9054\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u80B2\\u9054\\u79D1\\u5927', 299, true)&quot;); 
window.ddepartment.add(300,165,&quot;\u4E2D\u76DF-\u50D1\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement('\\u4E2D\\u76DF-\\u50D1\\u5149\\u79D1\\u5927', 300, true)&quot;); 
window.ddepartment.add(166,144,&quot;\u6821\u5916&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6821\\u5916', 166, true)&quot;); 
window.ddepartment.add(280,166,&quot;\u6821\u5916\u4EBA\u58EB&quot;, &quot;javascript:window.ddepartment.selectElement('\\u6821\\u5916\\u4EBA\\u58EB', 280, true)&quot;); 
window.ddepartment.add(481,0,&quot;test123&quot;, &quot;javascript:window.ddepartment.selectElement('test123', 481, true)&quot;); 
window.ddepartment.add(501,0,&quot;test234&quot;, &quot;javascript:window.ddepartment.selectElement('test234', 501, true)&quot;); 
window.ddepartment.add(502,0,&quot;test777&quot;, &quot;javascript:window.ddepartment.selectElement('test777', 502, true)&quot;); 
window.ddepartment.selectElement = function(lname, id, hideTree) { 
document.getElementById('department_0').value = id; 
document.getElementById('elementName').value = lname; 
if(hideTree == true) changeStatus('departmentTree'); 
}; 
 document.getElementById('departmentArea').innerHTML =  window.ddepartment; 
  
  
  










						
				
				
					代借帳號:
						
					
				
				
					流通停權:
					
					
					 電子郵件信箱(發送E-mail通知):
					
					
				
				
					
						OPAC登入停權:
					
					
						
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
				
				
					
						場地設備停權迄日:
					
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
					
					流通臺提示訊息:
					請修改密碼,帳密不要相同

					
						
					
				
				
					館員內部註記:
					
				
			
		
		
			
					
						
						
				
					新增日期:
					2018/02/14 14:38:51

					出生日期:
					
							
					
				
				
				
				
				
															
				
				讀者勾選同意時間:
					
													
				
				
				
					檢查日期:
					
					
				
				
					
					*可開始借閱日:
						

						*一般權限到期:
						
					
					
				
				
					 其他權限
					
				

				
				
					恢復預約權限日:
					

					較高權限到期:
					
				
				

				
				
					最高權限起自:
					

					最高權限到期:
					
				
				
				
					*預設值 
						所屬圖書館:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.ddefaultLocationId = new dTree('window.ddefaultLocationId', 'messages', '/inspireapp/images/'); 
window.ddefaultLocationId.add(0,-1,'館藏地'); 
window.ddefaultLocationId.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928', 1, true)&quot;); 
window.ddefaultLocationId.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('2 - 2', 463, true)&quot;); 
window.ddefaultLocationId.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('123 - 123', 583, true)&quot;); 
window.ddefaultLocationId.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('20230417 - 20230417', 1183, true)&quot;); 
window.ddefaultLocationId.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('20230418 - 20230418', 1203, true)&quot;); 
window.ddefaultLocationId.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('AH - \\u5B89\\u5357\\u91AB\\u9662', 167, true)&quot;); 
window.ddefaultLocationId.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340', 177, true)&quot;); 
window.ddefaultLocationId.add(643,1,&quot;av - av&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('av - av', 643, true)&quot;); 
window.ddefaultLocationId.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('B007 - B007', 303, true)&quot;); 
window.ddefaultLocationId.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BCSB4 - BCSB4', 883, true)&quot;); 
window.ddefaultLocationId.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BX - \\u53D6\\u66F8\\u6AC31', 823, true)&quot;); 
window.ddefaultLocationId.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BY - \\u53D6\\u66F8\\u6AC32', 903, true)&quot;); 
window.ddefaultLocationId.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CB - \\u5317\\u6E2F\\u5206\\u9928', 169, true)&quot;); 
window.ddefaultLocationId.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 179, true)&quot;); 
window.ddefaultLocationId.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 180, true)&quot;); 
window.ddefaultLocationId.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 181, true)&quot;); 
window.ddefaultLocationId.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340', 182, true)&quot;); 
window.ddefaultLocationId.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB', 183, true)&quot;); 
window.ddefaultLocationId.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('cbook - cbook', 623, true)&quot;); 
window.ddefaultLocationId.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('circd - circd', 624, true)&quot;); 
window.ddefaultLocationId.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('clp - clp', 683, true)&quot;); 
window.ddefaultLocationId.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662', 170, true)&quot;); 
window.ddefaultLocationId.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4', 211, true)&quot;); 
window.ddefaultLocationId.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928', 363, true)&quot;); 
window.ddefaultLocationId.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CU - \\u53F0\\u4E2D\\u7E3D\\u9928', 171, true)&quot;); 
window.ddefaultLocationId.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)', 603, true)&quot;); 
window.ddefaultLocationId.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 217, true)&quot;); 
window.ddefaultLocationId.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340', 218, true)&quot;); 
window.ddefaultLocationId.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44', 219, true)&quot;); 
window.ddefaultLocationId.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB', 220, true)&quot;); 
window.ddefaultLocationId.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 221, true)&quot;); 
window.ddefaultLocationId.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 244, true)&quot;); 
window.ddefaultLocationId.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340', 245, true)&quot;); 
window.ddefaultLocationId.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340', 246, true)&quot;); 
window.ddefaultLocationId.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340', 248, true)&quot;); 
window.ddefaultLocationId.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 250, true)&quot;); 
window.ddefaultLocationId.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340', 251, true)&quot;); 
window.ddefaultLocationId.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340', 252, true)&quot;); 
window.ddefaultLocationId.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457', 253, true)&quot;); 
window.ddefaultLocationId.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 256, true)&quot;); 
window.ddefaultLocationId.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 257, true)&quot;); 
window.ddefaultLocationId.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 258, true)&quot;); 
window.ddefaultLocationId.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 259, true)&quot;); 
window.ddefaultLocationId.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340', 261, true)&quot;); 
window.ddefaultLocationId.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 262, true)&quot;); 
window.ddefaultLocationId.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 263, true)&quot;); 
window.ddefaultLocationId.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4', 265, true)&quot;); 
window.ddefaultLocationId.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44', 266, true)&quot;); 
window.ddefaultLocationId.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3', 267, true)&quot;); 
window.ddefaultLocationId.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340', 270, true)&quot;); 
window.ddefaultLocationId.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8', 271, true)&quot;); 
window.ddefaultLocationId.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)', 272, true)&quot;); 
window.ddefaultLocationId.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4', 273, true)&quot;); 
window.ddefaultLocationId.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB', 274, true)&quot;); 
window.ddefaultLocationId.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3', 275, true)&quot;); 
window.ddefaultLocationId.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 7 - new item 7', 1103, true)&quot;); 
window.ddefaultLocationId.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599', 276, true)&quot;); 
window.ddefaultLocationId.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90', 3, true)&quot;); 
window.ddefaultLocationId.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('EB-P - EB-P', 345, true)&quot;); 
window.ddefaultLocationId.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('elect - elect', 648, true)&quot;); 
window.ddefaultLocationId.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('H-EQ - H-EQ', 343, true)&quot;); 
window.ddefaultLocationId.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('H-MR - H-MR', 344, true)&quot;); 
window.ddefaultLocationId.add(543,1,&quot;L - L&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('L - L', 543, true)&quot;); 
window.ddefaultLocationId.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('L40 - L40', 863, true)&quot;); 
window.ddefaultLocationId.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928', 1023, true)&quot;); 
window.ddefaultLocationId.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('LB-S - LB-S', 323, true)&quot;); 
window.ddefaultLocationId.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3', 173, true)&quot;); 
window.ddefaultLocationId.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4', 280, true)&quot;); 
window.ddefaultLocationId.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('LIB - LIB', 523, true)&quot;); 
window.ddefaultLocationId.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 1 - new item 1', 423, true)&quot;); 
window.ddefaultLocationId.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 3 - new item 3', 484, true)&quot;); 
window.ddefaultLocationId.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 10 - new item 10', 1283, true)&quot;); 
window.ddefaultLocationId.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 12 - new item 12', 1323, true)&quot;); 
window.ddefaultLocationId.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 13 - new item 13', 1343, true)&quot;); 
window.ddefaultLocationId.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 14 - new item 14', 1344, true)&quot;); 
window.ddefaultLocationId.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 16 - new item 16', 1264, true)&quot;); 
window.ddefaultLocationId.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 2 - new item 2', 483, true)&quot;); 
window.ddefaultLocationId.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 20 - new item 20', 1425, true)&quot;); 
window.ddefaultLocationId.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 4 - new item 4', 943, true)&quot;); 
window.ddefaultLocationId.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 5 - new item 5', 963, true)&quot;); 
window.ddefaultLocationId.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 6 - \\u82F1\\u624D\\u6821\\u5340', 1063, true)&quot;); 
window.ddefaultLocationId.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 8 - new item 8', 1243, true)&quot;); 
window.ddefaultLocationId.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 9 - new item 9', 1263, true)&quot;); 
window.ddefaultLocationId.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('NPTU - NPTU', 1043, true)&quot;); 
window.ddefaultLocationId.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('OUK - OUK', 503, true)&quot;); 
window.ddefaultLocationId.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('PT - \\u57F9\\u5FB7\\u91AB\\u9662', 174, true)&quot;); 
window.ddefaultLocationId.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 11 - new item 11', 1303, true)&quot;); 
window.ddefaultLocationId.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 17 - new item 17', 1363, true)&quot;); 
window.ddefaultLocationId.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340', 283, true)&quot;); 
window.ddefaultLocationId.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('ptext - ptext', 645, true)&quot;); 
window.ddefaultLocationId.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('SB3 - SB3', 1083, true)&quot;); 
window.ddefaultLocationId.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('T-P - T-P', 324, true)&quot;); 
window.ddefaultLocationId.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('TBBK - TBBK', 1403, true)&quot;); 
window.ddefaultLocationId.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('TH - \\u53F0\\u5317\\u5206\\u9662', 175, true)&quot;); 
window.ddefaultLocationId.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340', 284, true)&quot;); 
window.ddefaultLocationId.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340', 285, true)&quot;); 
window.ddefaultLocationId.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('W-P - W-P', 325, true)&quot;); 
window.ddefaultLocationId.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('YH - \\u8C50\\u539F\\u5206\\u9662', 176, true)&quot;); 
window.ddefaultLocationId.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 18 - new item 18', 1423, true)&quot;); 
window.ddefaultLocationId.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('new item 19 - new item 19', 1424, true)&quot;); 
window.ddefaultLocationId.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340', 286, true)&quot;); 
window.ddefaultLocationId.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('z3llc - z3llc', 983, true)&quot;); 
window.ddefaultLocationId.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('z6bkf - z6bkf', 647, true)&quot;); 
window.ddefaultLocationId.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('zd1a2 - zd1a2', 646, true)&quot;); 
window.ddefaultLocationId.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('zd1e - zd1e', 663, true)&quot;); 
window.ddefaultLocationId.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('zdlf - zdlf', 644, true)&quot;); 
window.ddefaultLocationId.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340', 403, true)&quot;); 
window.ddefaultLocationId.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF', 563, true)&quot;); 
window.ddefaultLocationId.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB', 1383, true)&quot;); 
window.ddefaultLocationId.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928', 383, true)&quot;); 
window.ddefaultLocationId.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340', 1384, true)&quot;); 
window.ddefaultLocationId.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement('\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928', 443, true)&quot;); 
window.ddefaultLocationId.selectElement = function(lname, id, hideTree) { 
document.getElementById('defaultLocationId_0').value = id; 
document.getElementById('elementName_0').value = lname; 
if(hideTree == true) changeStatus('defaultLocationIdTree'); 
if(lname) { tapestry.linkOnClick(document.getElementById('selectLink_0').href+'?sp=l'+id,'selectLink', false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById('selectLink_0').href+'?sp=l-1','selectLink', false); 
 } 
}; 
 document.getElementById('defaultLocationIdArea').innerHTML =  window.ddefaultLocationId; 
  
  
  



					*預設值 
						讀者身份類別
					
					
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

						
				
			
		
		
			
		
		
			
				
					
				
			
		
		
			
		
		
			
			修改/存檔
			
			
			
		
		
		
		
			
				借閱證號碼
				1234
			
		
	
	

 





 
  
    
    
    讀者證號資料
  
 
 
   



	

 查無資料


	


新證卡



 


 






	
		 
			
				   
				   
				   讀者悠遊卡資料
			 
		
		
	  		 



	

 查無資料


	


新悠遊卡


	 


		 
	







 
  
   
   
   讀者身分類別
  
 
 
   

	
	
	
		
		
		
			館藏地
		
			讀者身份類別
	
	
	
	
	









	
	
		 
			 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dSelectTreeStructure = new dTree('window.dSelectTreeStructure', 'messages', '/inspireapp/images/'); 
window.dSelectTreeStructure.add(0,-1,'館藏地'); 
window.dSelectTreeStructure.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928', 1, true)&quot;); 
window.dSelectTreeStructure.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('2 - 2', 463, true)&quot;); 
window.dSelectTreeStructure.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('123 - 123', 583, true)&quot;); 
window.dSelectTreeStructure.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('20230417 - 20230417', 1183, true)&quot;); 
window.dSelectTreeStructure.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('20230418 - 20230418', 1203, true)&quot;); 
window.dSelectTreeStructure.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('AH - \\u5B89\\u5357\\u91AB\\u9662', 167, true)&quot;); 
window.dSelectTreeStructure.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340', 177, true)&quot;); 
window.dSelectTreeStructure.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('av - av', 643, true)&quot;); 
window.dSelectTreeStructure.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('B007 - B007', 303, true)&quot;); 
window.dSelectTreeStructure.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BCSB4 - BCSB4', 883, true)&quot;); 
window.dSelectTreeStructure.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BX - \\u53D6\\u66F8\\u6AC31', 823, true)&quot;); 
window.dSelectTreeStructure.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BY - \\u53D6\\u66F8\\u6AC32', 903, true)&quot;); 
window.dSelectTreeStructure.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CB - \\u5317\\u6E2F\\u5206\\u9928', 169, true)&quot;); 
window.dSelectTreeStructure.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 179, true)&quot;); 
window.dSelectTreeStructure.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 180, true)&quot;); 
window.dSelectTreeStructure.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 181, true)&quot;); 
window.dSelectTreeStructure.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340', 182, true)&quot;); 
window.dSelectTreeStructure.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB', 183, true)&quot;); 
window.dSelectTreeStructure.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('cbook - cbook', 623, true)&quot;); 
window.dSelectTreeStructure.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('circd - circd', 624, true)&quot;); 
window.dSelectTreeStructure.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('clp - clp', 683, true)&quot;); 
window.dSelectTreeStructure.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662', 170, true)&quot;); 
window.dSelectTreeStructure.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4', 211, true)&quot;); 
window.dSelectTreeStructure.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928', 363, true)&quot;); 
window.dSelectTreeStructure.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CU - \\u53F0\\u4E2D\\u7E3D\\u9928', 171, true)&quot;); 
window.dSelectTreeStructure.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)', 603, true)&quot;); 
window.dSelectTreeStructure.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 217, true)&quot;); 
window.dSelectTreeStructure.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340', 218, true)&quot;); 
window.dSelectTreeStructure.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44', 219, true)&quot;); 
window.dSelectTreeStructure.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB', 220, true)&quot;); 
window.dSelectTreeStructure.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 221, true)&quot;); 
window.dSelectTreeStructure.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 244, true)&quot;); 
window.dSelectTreeStructure.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340', 245, true)&quot;); 
window.dSelectTreeStructure.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340', 246, true)&quot;); 
window.dSelectTreeStructure.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340', 248, true)&quot;); 
window.dSelectTreeStructure.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 250, true)&quot;); 
window.dSelectTreeStructure.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340', 251, true)&quot;); 
window.dSelectTreeStructure.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340', 252, true)&quot;); 
window.dSelectTreeStructure.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457', 253, true)&quot;); 
window.dSelectTreeStructure.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 256, true)&quot;); 
window.dSelectTreeStructure.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 257, true)&quot;); 
window.dSelectTreeStructure.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 258, true)&quot;); 
window.dSelectTreeStructure.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 259, true)&quot;); 
window.dSelectTreeStructure.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340', 261, true)&quot;); 
window.dSelectTreeStructure.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 262, true)&quot;); 
window.dSelectTreeStructure.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 263, true)&quot;); 
window.dSelectTreeStructure.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4', 265, true)&quot;); 
window.dSelectTreeStructure.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44', 266, true)&quot;); 
window.dSelectTreeStructure.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3', 267, true)&quot;); 
window.dSelectTreeStructure.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340', 270, true)&quot;); 
window.dSelectTreeStructure.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8', 271, true)&quot;); 
window.dSelectTreeStructure.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)', 272, true)&quot;); 
window.dSelectTreeStructure.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4', 273, true)&quot;); 
window.dSelectTreeStructure.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB', 274, true)&quot;); 
window.dSelectTreeStructure.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3', 275, true)&quot;); 
window.dSelectTreeStructure.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 7 - new item 7', 1103, true)&quot;); 
window.dSelectTreeStructure.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599', 276, true)&quot;); 
window.dSelectTreeStructure.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90', 3, true)&quot;); 
window.dSelectTreeStructure.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('EB-P - EB-P', 345, true)&quot;); 
window.dSelectTreeStructure.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('elect - elect', 648, true)&quot;); 
window.dSelectTreeStructure.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('H-EQ - H-EQ', 343, true)&quot;); 
window.dSelectTreeStructure.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('H-MR - H-MR', 344, true)&quot;); 
window.dSelectTreeStructure.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('L - L', 543, true)&quot;); 
window.dSelectTreeStructure.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('L40 - L40', 863, true)&quot;); 
window.dSelectTreeStructure.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928', 1023, true)&quot;); 
window.dSelectTreeStructure.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('LB-S - LB-S', 323, true)&quot;); 
window.dSelectTreeStructure.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3', 173, true)&quot;); 
window.dSelectTreeStructure.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4', 280, true)&quot;); 
window.dSelectTreeStructure.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('LIB - LIB', 523, true)&quot;); 
window.dSelectTreeStructure.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 1 - new item 1', 423, true)&quot;); 
window.dSelectTreeStructure.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 3 - new item 3', 484, true)&quot;); 
window.dSelectTreeStructure.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 10 - new item 10', 1283, true)&quot;); 
window.dSelectTreeStructure.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 12 - new item 12', 1323, true)&quot;); 
window.dSelectTreeStructure.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 13 - new item 13', 1343, true)&quot;); 
window.dSelectTreeStructure.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 14 - new item 14', 1344, true)&quot;); 
window.dSelectTreeStructure.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 16 - new item 16', 1264, true)&quot;); 
window.dSelectTreeStructure.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 2 - new item 2', 483, true)&quot;); 
window.dSelectTreeStructure.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 20 - new item 20', 1425, true)&quot;); 
window.dSelectTreeStructure.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 4 - new item 4', 943, true)&quot;); 
window.dSelectTreeStructure.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 5 - new item 5', 963, true)&quot;); 
window.dSelectTreeStructure.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 6 - \\u82F1\\u624D\\u6821\\u5340', 1063, true)&quot;); 
window.dSelectTreeStructure.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 8 - new item 8', 1243, true)&quot;); 
window.dSelectTreeStructure.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 9 - new item 9', 1263, true)&quot;); 
window.dSelectTreeStructure.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('NPTU - NPTU', 1043, true)&quot;); 
window.dSelectTreeStructure.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('OUK - OUK', 503, true)&quot;); 
window.dSelectTreeStructure.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('PT - \\u57F9\\u5FB7\\u91AB\\u9662', 174, true)&quot;); 
window.dSelectTreeStructure.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 11 - new item 11', 1303, true)&quot;); 
window.dSelectTreeStructure.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 17 - new item 17', 1363, true)&quot;); 
window.dSelectTreeStructure.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340', 283, true)&quot;); 
window.dSelectTreeStructure.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('ptext - ptext', 645, true)&quot;); 
window.dSelectTreeStructure.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('SB3 - SB3', 1083, true)&quot;); 
window.dSelectTreeStructure.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('T-P - T-P', 324, true)&quot;); 
window.dSelectTreeStructure.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('TBBK - TBBK', 1403, true)&quot;); 
window.dSelectTreeStructure.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('TH - \\u53F0\\u5317\\u5206\\u9662', 175, true)&quot;); 
window.dSelectTreeStructure.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340', 284, true)&quot;); 
window.dSelectTreeStructure.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340', 285, true)&quot;); 
window.dSelectTreeStructure.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('W-P - W-P', 325, true)&quot;); 
window.dSelectTreeStructure.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('YH - \\u8C50\\u539F\\u5206\\u9662', 176, true)&quot;); 
window.dSelectTreeStructure.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 18 - new item 18', 1423, true)&quot;); 
window.dSelectTreeStructure.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('new item 19 - new item 19', 1424, true)&quot;); 
window.dSelectTreeStructure.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340', 286, true)&quot;); 
window.dSelectTreeStructure.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('z3llc - z3llc', 983, true)&quot;); 
window.dSelectTreeStructure.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('z6bkf - z6bkf', 647, true)&quot;); 
window.dSelectTreeStructure.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('zd1a2 - zd1a2', 646, true)&quot;); 
window.dSelectTreeStructure.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('zd1e - zd1e', 663, true)&quot;); 
window.dSelectTreeStructure.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('zdlf - zdlf', 644, true)&quot;); 
window.dSelectTreeStructure.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340', 403, true)&quot;); 
window.dSelectTreeStructure.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF', 563, true)&quot;); 
window.dSelectTreeStructure.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB', 1383, true)&quot;); 
window.dSelectTreeStructure.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928', 383, true)&quot;); 
window.dSelectTreeStructure.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340', 1384, true)&quot;); 
window.dSelectTreeStructure.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement('\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928', 443, true)&quot;); 
window.dSelectTreeStructure.selectElement = function(lname, id, hideTree) { 
document.getElementById('SelectTreeStructure_0').value = id; 
document.getElementById('elementName_1').value = lname; 
if(hideTree == true) changeStatus('SelectTreeStructureTree'); 
}; 
 document.getElementById('SelectTreeStructureArea').innerHTML =  window.dSelectTreeStructure; 
  
  
  

   
    		
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

  	
	

	
	
		
			
				新增

		
	
	



 






 
  
   
   
   流通讀者狀態
  
 
 
 		  

window.suspendRight = function(data) {
	        var contextPath=$('#contextPath').text();
		    var printType=&quot;PREVIEW&quot;;
		    var code=data;
		    
		     var oForm = document.createElement(&quot;form&quot;);  
		     oForm.id=&quot;jasper&quot;;  
		     oForm.method=&quot;post&quot;;  
		     oForm.action=contextPath+&quot;struts/suspendRight.jsp&quot;;  
		     
		     var keys=['codeX'];  
		     var values=[code];
		       
			 if (keys &amp;&amp; values &amp;&amp; (keys.length == values.length)){  
			    for (var i=0; i &lt; keys.length; i++){  
			        var oInput = document.createElement(&quot;input&quot;);  
			        oInput.type=&quot;hidden&quot;;  
			        oInput.name=keys[i];  
			        oInput.value=values[i];  
			        oForm.appendChild(oInput);  
			    }
			 }
			 if('PREVIEW'==printType){
			     oForm.target=&quot;jasperTarget&quot;;
			     oForm.onSubmit=function(){openSpecfiyWindown(&quot;jasperWindown&quot;)};
		     }
		     document.body.appendChild(oForm);  
		     oForm.submit();   
}




(function($){$(document).ready(function(){$(&quot;#borrowedFlag&quot;).hide();if($(&quot;#isLanguage&quot;).html()==&quot;1&quot;){$(&quot;#borrowedFlag&quot;).show();}});})(jQuery);
 


var patronItemDetails = document.getElementById(&quot;PatronItemDetails&quot;);
var isShowPatronPhotoLoanDesk = document.getElementById(&quot;isShowPatronPhotoLoanDesk&quot;);
var reset = document.getElementById(&quot;reset&quot;);
if(isShowPatronPhotoLoanDesk){
	patronItemDetails.style.marginBottom = &quot;-49px&quot;;
    patronItemDetails.style.marginTop = &quot;-72px&quot;;
    reset.style.marginTop = &quot;-77px&quot;;
}


	.tdlwx{ font-weight:700; font-style:italic; font-family:Verdana, Geneva, sans-serif;white-space:nowrap; width:35px;}


	1

	/inspireapp/


	
	
		
			
				未選擇讀者資料
		
	
	

	

	
	    
	    
 
  
    罰款
  
  
 
 
  
 

	
	
		
	    
 
  
    Penalties
  
  
 
 
  
 

	



 
 
  
    預約可取
  
  
 
 
  
 


 
 
  
    逾期罰金估算
  
  
 
 
  
 




 
  
    讀者基本資訊
  
  
 
 
  
 




 
 







	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;patronEditForm&quot;);
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }
calendar_lockedDatetimeDatePicker = new Calendar();

	
calendar_lockedDatetimeDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_lockedDatetimeDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).lockedDatetimeDatePicker;
  var value = calendar_lockedDatetimeDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_equEndDateDatePicker = new Calendar();

	
calendar_equEndDateDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_equEndDateDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).equEndDateDatePicker;
  var value = calendar_equEndDateDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

tapestry.form.registerForm(&quot;classesForm&quot;);

closeDialogComponent('PenaltiesDialog');
closeDialogComponent('SiteEquPenaltiesDialog');
closeDialogComponent('HoldsListDialog');
closeDialogComponent('estimatedPenaltiesArea');
closeDialogComponent('detailsDialog');
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
closeDialogComponent('TinreadMessageDialog');

tapestry.form.focusField('TextField');});
// -->




&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear/html[1]</value>
      <webElementGuid>56c88489-1fa7-4216-8dda-3bf069fd3511</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>bfc62580-58a5-4003-8698-7e7386880dba</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>1863e5f3-7a3a-491e-9cc6-af93c5b2a49d</webElementGuid>
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

var calendar_lockedDatetimeDatePicker;
var calendar_equEndDateDatePicker;
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

 

	

	
		
		  
		    
	
		讀者資料維護
		
		
			
		        讀者基本資料
			
		
	

		  
		  
		  
		    
			
			
		  
		


	
	     


 


 
  
   
    
    
    一般細節
   
  
  
 
 
  
	
		
	
	
	
	
	
		$(document).keypress(function(e) {
			if (e.which == 13) {
				return false;
			}
		});
	
	
		(function($) {
			//日期選擇器
			function initDatePicker() {
		        var contextPath = trim($(&quot; , &quot;'&quot; , &quot;#contextPath&quot; , &quot;'&quot; , &quot;).text());
		        $(&quot; , &quot;'&quot; , &quot;.DatePickerID&quot; , &quot;'&quot; , &quot;).datepick({
		            dateFormat: &quot; , &quot;'&quot; , &quot;yy/mm/dd&quot; , &quot;'&quot; , &quot;,
		            showOn: &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;,
		            buttonImageOnly: true,
		            buttonImage: contextPath + &quot; , &quot;'&quot; , &quot;/js/datepick/DatePickerIcon.png&quot; , &quot;'&quot; , &quot;,
		            yearRange: &quot; , &quot;'&quot; , &quot;c-100:c+50&quot; , &quot;'&quot; , &quot;
		        });
		    }
		    // 在文檔準備好時初始化日期選擇器
		    $(document).ready(function() {
		        initDatePicker();
		    });
		 	// 使用MutationObserver監聽文檔內容的變化
		    var observer = new MutationObserver(function(mutations) {
		        mutations.forEach(function(mutation) {
		            if (mutation.type === &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
		                // 文檔內容發生變化時重新初始化日期選擇器
		                initDatePicker();
		            }
		        });
		    });
		    // 監聽整個文檔的變化
		    observer.observe(document, { childList: true, subtree: true });
		})(jQuery);
	
	
		(function($) {
			$(document).ready(function() {
				$(&quot;#dateToBeCheckedflag&quot;).hide();
				if ($(&quot;#isLanguage&quot;).html() == &quot;1&quot;) {
					$(&quot;#dateToBeCheckedflag&quot;).show();
				}
			});
		})(jQuery);
	
	

#loading-overlay { position: absolute; z-index: 2147483647; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: -500; background-color: transparent; opacity: 0.7; }

	




















		 
	function blockScreen_0(e){
		var iDiv = window.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
		iDiv.id = &quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;; 
		var blockDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv_0&quot; , &quot;'&quot; , &quot;);
		if (blockDiv!=null) blockDiv.appendChild(iDiv);
	} 
	document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function() {
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButton&quot;), &quot;onclick&quot;, blockScreen_0);
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButtonWithQuest&quot;), &quot;onclick&quot;, blockScreen_0);
	}, false);





//&lt;![CDATA[

	 function deleteblock_0(){
		var parent = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv_0&quot; , &quot;'&quot; , &quot;);
		var child = document.getElementById(&quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;);		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock_0();
 
//]]&gt;




	
		輸入欄位驗証錯誤提示行動電話電話號碼格式錯誤 
		1
		
			/inspireapp/
		
		
			   
		
		
		
		
			
						
				
					*姓名:
					

					暱稱:
					
				
				
					*帳號:
					
							
						 
							   1234  帳號修改
							
						

					* 身分證號:
					
				
				
					行動電話:
					







					讀者狀態:
					
有效讀者
離職離校
微波通訊
MATLAB程式設計
無線行動網路架構
電力電子學
硬體描述語言設計
電路板設計實務
PHP程式設計
通訊實習
手持式裝置設計與應用
進階數學
半導體元件及量測實務
科技英文
創意與專利1

				
				
					單位所系:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
所系單位_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree(&quot; , &quot;'&quot; , &quot;window.ddepartment&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(0,-1,&quot; , &quot;'&quot; , &quot;所系單位&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(441,0,&quot;_New Item&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;_New Item&quot; , &quot;'&quot; , &quot;, 441, true)&quot;); 
window.ddepartment.add(461,0,&quot;_New Item1234&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;_New Item1234&quot; , &quot;'&quot; , &quot;, 461, true)&quot;); 
window.ddepartment.add(121,0,&quot;\u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 121, true)&quot;); 
window.ddepartment.add(141,121,&quot;\u884C\u653F\u55AE\u4F4D\u5176\u4ED6\u5176\u4ED6\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u884C\\u653F\\u55AE\\u4F4D\\u5176\\u4ED6\\u5176\\u4ED6\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 141, true)&quot;); 
window.ddepartment.add(145,141,&quot;\u5B78\u6821\u884C\u653F&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u884C\\u653F&quot; , &quot;'&quot; , &quot;, 145, true)&quot;); 
window.ddepartment.add(167,145,&quot;\u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.ddepartment.add(168,145,&quot;\u5317\u6E2F\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 168, true)&quot;); 
window.ddepartment.add(122,145,&quot;\u5B89\u5357\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B89\\u5357\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 122, true)&quot;); 
window.ddepartment.add(123,145,&quot;\u6C34\u6E73\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6C34\\u6E73\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 123, true)&quot;); 
window.ddepartment.add(124,145,&quot;\u5B78\u6821\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 124, true)&quot;); 
window.ddepartment.add(125,145,&quot;\u5317\u6E2F\u5206\u90E8\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u5206\\u90E8\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 125, true)&quot;); 
window.ddepartment.add(126,145,&quot;\u5B78\u6821\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u7814\\u7A76\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 126, true)&quot;); 
window.ddepartment.add(142,121,&quot;\u5B78\u8853\u55AE\u4F4D\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u8853\\u55AE\\u4F4D\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 142, true)&quot;); 
window.ddepartment.add(146,142,&quot;\u91AB\u5B78\u9662\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u9662\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 146, true)&quot;); 
window.ddepartment.add(127,146,&quot;\u91AB\u5B78\u7CFB\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7CFB\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 127, true)&quot;); 
window.ddepartment.add(401,127,&quot;\u91AB\u5B78\u7CFB\u4E00\u5E74A\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7CFB\\u4E00\\u5E74A\\u73ED&quot; , &quot;'&quot; , &quot;, 401, true)&quot;); 
window.ddepartment.add(128,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 128, true)&quot;); 
window.ddepartment.add(129,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 129, true)&quot;); 
window.ddepartment.add(130,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 130, true)&quot;); 
window.ddepartment.add(131,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 131, true)&quot;); 
window.ddepartment.add(132,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 132, true)&quot;); 
window.ddepartment.add(133,146,&quot;\u570B\u969B\u751F\u7269\u91AB\u5B78\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u570B\\u969B\\u751F\\u7269\\u91AB\\u5B78\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 133, true)&quot;); 
window.ddepartment.add(134,146,&quot;\u81E8\u5E8A\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u81E8\\u5E8A\\u91AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 134, true)&quot;); 
window.ddepartment.add(135,146,&quot;\u57FA\u790E\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u57FA\\u790E\\u91AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 135, true)&quot;); 
window.ddepartment.add(136,146,&quot;\u514D\u75AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u514D\\u75AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 136, true)&quot;); 
window.ddepartment.add(137,146,&quot;\u764C\u75C7\u751F\u7269\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u751F\\u7269\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 137, true)&quot;); 
window.ddepartment.add(138,146,&quot;\u795E\u7D93\u79D1\u5B78\u8207\u8A8D\u77E5\u79D1\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u79D1\\u5B78\\u8207\\u8A8D\\u77E5\\u79D1\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 138, true)&quot;); 
window.ddepartment.add(139,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 139, true)&quot;); 
window.ddepartment.add(169,146,&quot;\u764C\u75C7\u751F\u7269\u8207\u85E5\u7269\u7814\u767C\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u751F\\u7269\\u8207\\u85E5\\u7269\\u7814\\u767C\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.ddepartment.add(170,146,&quot;\u8001\u5316\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8001\\u5316\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.ddepartment.add(171,146,&quot;\u8F49\u8B6F\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8F49\\u8B6F\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.ddepartment.add(321,146,&quot;\u751F\u91AB\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u91AB\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 321, true)&quot;); 
window.ddepartment.add(147,142,&quot;\u4E2D\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 147, true)&quot;); 
window.ddepartment.add(421,147,&quot;New Item&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;New Item&quot; , &quot;'&quot; , &quot;, 421, true)&quot;); 
window.ddepartment.add(172,147,&quot;\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 172, true)&quot;); 
window.ddepartment.add(173,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u7532\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u7532\\u7D44&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.ddepartment.add(174,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u4E59\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u4E59\\u7D44&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.ddepartment.add(175,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.ddepartment.add(176,147,&quot;\u5B78\u58EB\u5F8C\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u58EB\\u5F8C\\u4E2D\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.ddepartment.add(177,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.ddepartment.add(178,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 178, true)&quot;); 
window.ddepartment.add(140,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 140, true)&quot;); 
window.ddepartment.add(181,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.ddepartment.add(182,147,&quot;\u570B\u969B\u91DD\u7078\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u570B\\u969B\\u91DD\\u7078\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.ddepartment.add(183,147,&quot;\u4E2D\u7378\u91AB\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u7378\\u91AB\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.ddepartment.add(184,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 184, true)&quot;); 
window.ddepartment.add(185,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 185, true)&quot;); 
window.ddepartment.add(186,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 186, true)&quot;); 
window.ddepartment.add(187,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 187, true)&quot;); 
window.ddepartment.add(148,142,&quot;\u85E5\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 148, true)&quot;); 
window.ddepartment.add(179,148,&quot;\u85E5\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.ddepartment.add(180,148,&quot;\u85E5\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.ddepartment.add(201,148,&quot;\u85E5\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 201, true)&quot;); 
window.ddepartment.add(202,148,&quot;\u751F\u6280\u88FD\u85E5\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u6280\\u88FD\\u85E5\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 202, true)&quot;); 
window.ddepartment.add(149,142,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 149, true)&quot;); 
window.ddepartment.add(203,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 203, true)&quot;); 
window.ddepartment.add(204,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 204, true)&quot;); 
window.ddepartment.add(205,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 205, true)&quot;); 
window.ddepartment.add(206,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662\u5927\u4E00\u4E0D\u5206\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662\\u5927\\u4E00\\u4E0D\\u5206\\u7CFB&quot; , &quot;'&quot; , &quot;, 206, true)&quot;); 
window.ddepartment.add(207,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 207, true)&quot;); 
window.ddepartment.add(208,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 208, true)&quot;); 
window.ddepartment.add(209,149,&quot;\u516C\u5171\u885B\u751F\u570B\u969B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u570B\\u969B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 209, true)&quot;); 
window.ddepartment.add(210,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 210, true)&quot;); 
window.ddepartment.add(211,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.ddepartment.add(212,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 212, true)&quot;); 
window.ddepartment.add(213,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 213, true)&quot;); 
window.ddepartment.add(214,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 214, true)&quot;); 
window.ddepartment.add(215,149,&quot;\u751F\u7269\u7D71\u8A08\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u7D71\\u8A08\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 215, true)&quot;); 
window.ddepartment.add(216,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 216, true)&quot;); 
window.ddepartment.add(381,149,&quot;\u55AE\u4F4D\u7CFB\u6240\u532F\u5165&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u55AE\\u4F4D\\u7CFB\\u6240\\u532F\\u5165&quot; , &quot;'&quot; , &quot;, 381, true)&quot;); 
window.ddepartment.add(191,149,&quot;\u91AB\u5B78\u5DE5\u7A0B\u8207\u5FA9\u5065\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u5DE5\\u7A0B\\u8207\\u5FA9\\u5065\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 191, true)&quot;); 
window.ddepartment.add(245,149,&quot;\u751F\u7269\u91AB\u5B78\u5DE5\u7A0B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5DE5\\u7A0B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.ddepartment.add(150,142,&quot;\u5065\u5EB7\u7167\u8B77\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u7167\\u8B77\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 150, true)&quot;); 
window.ddepartment.add(217,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.ddepartment.add(218,150,&quot;\u8B77\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.ddepartment.add(219,150,&quot;\u904B\u52D5\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u904B\\u52D5\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.ddepartment.add(220,150,&quot;\u53E3\u8154\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53E3\\u8154\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.ddepartment.add(221,150,&quot;\u4E8C\u5E74\u5236\u8B77\u7406\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E8C\\u5E74\\u5236\\u8B77\\u7406\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.ddepartment.add(188,150,&quot;\u4E8C\u5E74\u5236\u547C\u5438\u6CBB\u7642\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E8C\\u5E74\\u5236\\u547C\\u5438\\u6CBB\\u7642\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 188, true)&quot;); 
window.ddepartment.add(189,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB\u5FA9\u5065\u79D1\u5B78\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB\\u5FA9\\u5065\\u79D1\\u5B78\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 189, true)&quot;); 
window.ddepartment.add(190,150,&quot;\u8B77\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 190, true)&quot;); 
window.ddepartment.add(361,150,&quot;\u8B77\u7406\u5B78\u7CFB\u8DE8\u9818\u57DF\u9577\u671F\u7167\u8B77\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB\\u8DE8\\u9818\\u57DF\\u9577\\u671F\\u7167\\u8B77\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 361, true)&quot;); 
window.ddepartment.add(192,150,&quot;\u5065\u5EB7\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 192, true)&quot;); 
window.ddepartment.add(151,142,&quot;\u751F\u6280\u88FD\u85E5\u66A8\u98DF\u54C1\u79D1\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u6280\\u88FD\\u85E5\\u66A8\\u98DF\\u54C1\\u79D1\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 151, true)&quot;); 
window.ddepartment.add(193,151,&quot;\u71DF\u990A\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 193, true)&quot;); 
window.ddepartment.add(194,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 194, true)&quot;); 
window.ddepartment.add(195,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 195, true)&quot;); 
window.ddepartment.add(196,151,&quot;\u71DF\u990A\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 196, true)&quot;); 
window.ddepartment.add(197,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 197, true)&quot;); 
window.ddepartment.add(198,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 198, true)&quot;); 
window.ddepartment.add(199,151,&quot;\u88FD\u85E5\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u88FD\\u85E5\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 199, true)&quot;); 
window.ddepartment.add(200,151,&quot;\u98DF\u54C1\u66A8\u85E5\u7269\u5B89\u5168\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u98DF\\u54C1\\u66A8\\u85E5\\u7269\\u5B89\\u5168\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 200, true)&quot;); 
window.ddepartment.add(241,151,&quot;\u71DF\u990A\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 241, true)&quot;); 
window.ddepartment.add(242,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 242, true)&quot;); 
window.ddepartment.add(243,151,&quot;\u65B0\u85E5\u958B\u767C\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u65B0\\u85E5\\u958B\\u767C\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 243, true)&quot;); 
window.ddepartment.add(244,151,&quot;\u751F\u7269\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.ddepartment.add(152,142,&quot;\u4EBA\u6587\u8207\u79D1\u6280\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4EBA\\u6587\\u8207\\u79D1\\u6280\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 152, true)&quot;); 
window.ddepartment.add(322,152,&quot;\u79D1\u6280\u6CD5\u5F8B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u79D1\\u6280\\u6CD5\\u5F8B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 322, true)&quot;); 
window.ddepartment.add(362,152,&quot;\u79D1\u6280\u7BA1\u7406\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u79D1\\u6280\\u7BA1\\u7406\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 362, true)&quot;); 
window.ddepartment.add(153,142,&quot;\u7259\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 153, true)&quot;); 
window.ddepartment.add(246,153,&quot;\u7259\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.ddepartment.add(247,153,&quot;\u7259\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 247, true)&quot;); 
window.ddepartment.add(363,153,&quot;\u7259\u91AB\u5B78\u7CFB\u53E3\u8154\u91AB\u5B78\u7522\u696D\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u53E3\\u8154\\u91AB\\u5B78\\u7522\\u696D\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.ddepartment.add(323,153,&quot;\u7259\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.ddepartment.add(154,142,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 154, true)&quot;); 
window.ddepartment.add(248,154,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.ddepartment.add(143,121,&quot;\u9644\u8A2D\u6A5F\u69CB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u8A2D\\u6A5F\\u69CB&quot; , &quot;'&quot; , &quot;, 143, true)&quot;); 
window.ddepartment.add(222,143,&quot;\u4E2D\u570B\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u9644\\u91AB&quot; , &quot;'&quot; , &quot;, 222, true)&quot;); 
window.ddepartment.add(223,222,&quot;\u9644\u91AB\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB\\u7814\\u7A76\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 223, true)&quot;); 
window.ddepartment.add(224,222,&quot;\u5167\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5167\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 224, true)&quot;); 
window.ddepartment.add(225,222,&quot;\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5916\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 225, true)&quot;); 
window.ddepartment.add(226,222,&quot;\u795E\u7D93\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u5916\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 226, true)&quot;); 
window.ddepartment.add(249,222,&quot;\u9AA8\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9AA8\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 249, true)&quot;); 
window.ddepartment.add(250,222,&quot;\u6CCC\u5C3F\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6CCC\\u5C3F\\u90E8&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.ddepartment.add(251,222,&quot;\u5A66\u7522\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5A66\\u7522\\u90E8&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.ddepartment.add(227,222,&quot;\u795E\u7D93\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u90E8&quot; , &quot;'&quot; , &quot;, 227, true)&quot;); 
window.ddepartment.add(228,222,&quot;\u8033\u9F3B\u5589\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8033\\u9F3B\\u5589\\u90E8&quot; , &quot;'&quot; , &quot;, 228, true)&quot;); 
window.ddepartment.add(229,222,&quot;\u76AE\u819A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u76AE\\u819A\\u79D1&quot; , &quot;'&quot; , &quot;, 229, true)&quot;); 
window.ddepartment.add(230,222,&quot;\u7259\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u90E8&quot; , &quot;'&quot; , &quot;, 230, true)&quot;); 
window.ddepartment.add(231,222,&quot;\u7CBE\u795E\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7CBE\\u795E\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 231, true)&quot;); 
window.ddepartment.add(232,222,&quot;\u5FA9\u5065\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5FA9\\u5065\\u90E8&quot; , &quot;'&quot; , &quot;, 232, true)&quot;); 
window.ddepartment.add(233,222,&quot;\u9EBB\u9189\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9EBB\\u9189\\u90E8&quot; , &quot;'&quot; , &quot;, 233, true)&quot;); 
window.ddepartment.add(235,222,&quot;\u81E8\u5E8A\u71DF\u990A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u81E8\\u5E8A\\u71DF\\u990A\\u79D1&quot; , &quot;'&quot; , &quot;, 235, true)&quot;); 
window.ddepartment.add(234,222,&quot;\u4E2D\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u90E8&quot; , &quot;'&quot; , &quot;, 234, true)&quot;); 
window.ddepartment.add(252,222,&quot;\u4E2D\u570B\u9644\u91AB\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u9644\\u91AB\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.ddepartment.add(253,222,&quot;\u793E\u6703\u5DE5\u4F5C\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u793E\\u6703\\u5DE5\\u4F5C\\u5BA4&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.ddepartment.add(254,222,&quot;\u773C\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u773C\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 254, true)&quot;); 
window.ddepartment.add(255,222,&quot;\u5152\u7AE5\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5152\\u7AE5\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 255, true)&quot;); 
window.ddepartment.add(256,222,&quot;\u75C5\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u75C5\\u7406\\u90E8&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.ddepartment.add(257,222,&quot;\u57FA\u56E0\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u57FA\\u56E0\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.ddepartment.add(258,222,&quot;\u9810\u9632\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9810\\u9632\\u91AB\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.ddepartment.add(259,222,&quot;\u91AB\u5B78\u7814\u7A76\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7814\\u7A76\\u90E8&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.ddepartment.add(260,222,&quot;\u6559\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6559\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 260, true)&quot;); 
window.ddepartment.add(261,222,&quot;\u6025\u75C7\u66A8\u5916\u50B7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6025\\u75C7\\u66A8\\u5916\\u50B7\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.ddepartment.add(262,222,&quot;\u8B77\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u90E8&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.ddepartment.add(263,222,&quot;\u85E5\u5291\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5291\\u90E8&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.ddepartment.add(264,222,&quot;\u91AB\u5B78\u5F71\u50CF\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u5F71\\u50CF\\u90E8&quot; , &quot;'&quot; , &quot;, 264, true)&quot;); 
window.ddepartment.add(265,222,&quot;\u6AA2\u9A57\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6AA2\\u9A57\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.ddepartment.add(266,222,&quot;\u6838\u5B50\u91AB\u5B78\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6838\\u5B50\\u91AB\\u5B78\\u79D1&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.ddepartment.add(267,222,&quot;\u795E\u7D93\u7CBE\u795E\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u7CBE\\u795E\\u91AB\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.ddepartment.add(268,222,&quot;\u91AB\u7642\u54C1\u8CEA\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u7642\\u54C1\\u8CEA\\u90E8&quot; , &quot;'&quot; , &quot;, 268, true)&quot;); 
window.ddepartment.add(269,222,&quot;\u764C\u75C7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 269, true)&quot;); 
window.ddepartment.add(155,143,&quot;\u9644\u91AB-\u5317\u6E2F\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5317\\u6E2F\\u9644\\u91AB&quot; , &quot;'&quot; , &quot;, 155, true)&quot;); 
window.ddepartment.add(270,155,&quot;\u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.ddepartment.add(156,143,&quot;\u9644\u91AB-\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 156, true)&quot;); 
window.ddepartment.add(271,156,&quot;\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.ddepartment.add(157,143,&quot;\u9644\u91AB-\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4&quot; , &quot;'&quot; , &quot;, 157, true)&quot;); 
window.ddepartment.add(272,157,&quot;\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.ddepartment.add(158,143,&quot;\u9644\u91AB-\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 158, true)&quot;); 
window.ddepartment.add(273,158,&quot;\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.ddepartment.add(159,143,&quot;\u9644\u91AB-\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 159, true)&quot;); 
window.ddepartment.add(274,159,&quot;\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.ddepartment.add(160,143,&quot;\u9644\u91AB-\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 160, true)&quot;); 
window.ddepartment.add(275,160,&quot;\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.ddepartment.add(301,143,&quot;\u9644\u91AB-\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240&quot; , &quot;'&quot; , &quot;, 301, true)&quot;); 
window.ddepartment.add(302,301,&quot;\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240&quot; , &quot;'&quot; , &quot;, 302, true)&quot;); 
window.ddepartment.add(161,143,&quot;\u9644\u91AB-\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8349\\u5C6F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 161, true)&quot;); 
window.ddepartment.add(276,161,&quot;\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8349\\u5C6F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.ddepartment.add(162,143,&quot;\u9644\u91AB-\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 162, true)&quot;); 
window.ddepartment.add(277,162,&quot;\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 277, true)&quot;); 
window.ddepartment.add(163,143,&quot;\u9644\u91AB-\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 163, true)&quot;); 
window.ddepartment.add(278,163,&quot;\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 278, true)&quot;); 
window.ddepartment.add(164,143,&quot;\u9644\u91AB-\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 164, true)&quot;); 
window.ddepartment.add(279,164,&quot;\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 279, true)&quot;); 
window.ddepartment.add(144,121,&quot;\u6821\u5916\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 144, true)&quot;); 
window.ddepartment.add(165,144,&quot;\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9928\\u969B\\u5408\\u4F5C&quot; , &quot;'&quot; , &quot;, 165, true)&quot;); 
window.ddepartment.add(236,165,&quot;NDDS\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;NDDS\\u9928\\u969B\\u5408\\u4F5C&quot; , &quot;'&quot; , &quot;, 236, true)&quot;); 
window.ddepartment.add(237,165,&quot;\u4E92\u501F\u5354\u8B70\u806F\u76DF&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E92\\u501F\\u5354\\u8B70\\u806F\\u76DF&quot; , &quot;'&quot; , &quot;, 237, true)&quot;); 
window.ddepartment.add(238,165,&quot;\u4E2D\u76DF-\u5927\u8449\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5927\\u8449\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 238, true)&quot;); 
window.ddepartment.add(239,165,&quot;\u4E2D\u76DF-\u4E2D\u5C71\u91AB\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u5C71\\u91AB\\u5927&quot; , &quot;'&quot; , &quot;, 239, true)&quot;); 
window.ddepartment.add(240,165,&quot;\u4E2D\u76DF-\u4E2D\u81FA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u81FA\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 240, true)&quot;); 
window.ddepartment.add(281,165,&quot;\u4E2D\u76DF-\u4E2D\u8208\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u8208\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 281, true)&quot;); 
window.ddepartment.add(282,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u6559\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u6559\\u5927&quot; , &quot;'&quot; , &quot;, 282, true)&quot;); 
window.ddepartment.add(283,165,&quot;\u4E2D\u76DF-\u5F18\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5F18\\u5149\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.ddepartment.add(284,165,&quot;\u4E2D\u76DF-\u4E9E\u6D32\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E9E\\u6D32\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.ddepartment.add(285,165,&quot;\u4E2D\u76DF-\u6771\u6D77\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u6771\\u6D77\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.ddepartment.add(286,165,&quot;\u4E2D\u76DF-\u5EFA\u570B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5EFA\\u570B\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.ddepartment.add(287,165,&quot;\u4E2D\u76DF-\u66A8\u5357\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u66A8\\u5357\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 287, true)&quot;); 
window.ddepartment.add(288,165,&quot;\u4E2D\u76DF-\u9022\u7532\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u9022\\u7532\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 288, true)&quot;); 
window.ddepartment.add(289,165,&quot;\u4E2D\u76DF-\u671D\u967D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u671D\\u967D\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 289, true)&quot;); 
window.ddepartment.add(290,165,&quot;\u4E2D\u76DF-\u52E4\u76CA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u52E4\\u76CA\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 290, true)&quot;); 
window.ddepartment.add(291,165,&quot;\u4E2D\u76DF-\u5F70\u5316\u5E2B\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5F70\\u5316\\u5E2B\\u5927&quot; , &quot;'&quot; , &quot;, 291, true)&quot;); 
window.ddepartment.add(292,165,&quot;\u4E2D\u76DF-\u975C\u5B9C\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u975C\\u5B9C\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 292, true)&quot;); 
window.ddepartment.add(293,165,&quot;\u4E2D\u76DF-\u5DBA\u6771\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5DBA\\u6771\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 293, true)&quot;); 
window.ddepartment.add(294,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 294, true)&quot;); 
window.ddepartment.add(295,165,&quot;\u4E2D\u76DF-\u806F\u5408\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u806F\\u5408\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 295, true)&quot;); 
window.ddepartment.add(296,165,&quot;\u4E2D\u76DF-\u660E\u9053\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u660E\\u9053\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 296, true)&quot;); 
window.ddepartment.add(297,165,&quot;\u4E2D\u76DF-\u5357\u958B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5357\\u958B\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 297, true)&quot;); 
window.ddepartment.add(298,165,&quot;\u4E2D\u76DF-\u4FEE\u5E73\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4FEE\\u5E73\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 298, true)&quot;); 
window.ddepartment.add(299,165,&quot;\u4E2D\u76DF-\u80B2\u9054\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u80B2\\u9054\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 299, true)&quot;); 
window.ddepartment.add(300,165,&quot;\u4E2D\u76DF-\u50D1\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u50D1\\u5149\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 300, true)&quot;); 
window.ddepartment.add(166,144,&quot;\u6821\u5916&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916&quot; , &quot;'&quot; , &quot;, 166, true)&quot;); 
window.ddepartment.add(280,166,&quot;\u6821\u5916\u4EBA\u58EB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916\\u4EBA\\u58EB&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.ddepartment.add(481,0,&quot;test123&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test123&quot; , &quot;'&quot; , &quot;, 481, true)&quot;); 
window.ddepartment.add(501,0,&quot;test234&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test234&quot; , &quot;'&quot; , &quot;, 501, true)&quot;); 
window.ddepartment.add(502,0,&quot;test777&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test777&quot; , &quot;'&quot; , &quot;, 502, true)&quot;); 
window.ddepartment.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;department_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;departmentTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;departmentArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ddepartment; 
  
  
  










						
				
				
					代借帳號:
						
					
				
				
					流通停權:
					
					
					 電子郵件信箱(發送E-mail通知):
					
					
				
				
					
						OPAC登入停權:
					
					
						
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
				
				
					
						場地設備停權迄日:
					
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
					
					流通臺提示訊息:
					請修改密碼,帳密不要相同

					
						
					
				
				
					館員內部註記:
					
				
			
		
		
			
					
						
						
				
					新增日期:
					2018/02/14 14:38:51

					出生日期:
					
							
					
				
				
				
				
				
															
				
				讀者勾選同意時間:
					
													
				
				
				
					檢查日期:
					
					
				
				
					
					*可開始借閱日:
						

						*一般權限到期:
						
					
					
				
				
					 其他權限
					
				

				
				
					恢復預約權限日:
					

					較高權限到期:
					
				
				

				
				
					最高權限起自:
					

					最高權限到期:
					
				
				
				
					*預設值 
						所屬圖書館:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.ddefaultLocationId = new dTree(&quot; , &quot;'&quot; , &quot;window.ddefaultLocationId&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddefaultLocationId.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.ddefaultLocationId.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.ddefaultLocationId.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.ddefaultLocationId.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.ddefaultLocationId.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.ddefaultLocationId.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.ddefaultLocationId.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.ddefaultLocationId.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.ddefaultLocationId.add(643,1,&quot;av - av&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.ddefaultLocationId.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.ddefaultLocationId.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.ddefaultLocationId.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.ddefaultLocationId.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.ddefaultLocationId.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.ddefaultLocationId.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.ddefaultLocationId.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.ddefaultLocationId.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.ddefaultLocationId.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.ddefaultLocationId.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.ddefaultLocationId.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.ddefaultLocationId.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.ddefaultLocationId.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.ddefaultLocationId.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.ddefaultLocationId.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.ddefaultLocationId.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.ddefaultLocationId.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.ddefaultLocationId.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.ddefaultLocationId.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.ddefaultLocationId.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.ddefaultLocationId.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.ddefaultLocationId.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.ddefaultLocationId.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.ddefaultLocationId.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.ddefaultLocationId.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.ddefaultLocationId.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.ddefaultLocationId.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.ddefaultLocationId.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.ddefaultLocationId.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.ddefaultLocationId.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.ddefaultLocationId.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.ddefaultLocationId.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.ddefaultLocationId.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.ddefaultLocationId.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.ddefaultLocationId.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.ddefaultLocationId.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.ddefaultLocationId.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.ddefaultLocationId.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.ddefaultLocationId.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.ddefaultLocationId.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.ddefaultLocationId.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.ddefaultLocationId.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.ddefaultLocationId.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.ddefaultLocationId.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.ddefaultLocationId.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.ddefaultLocationId.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.ddefaultLocationId.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.ddefaultLocationId.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.ddefaultLocationId.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.ddefaultLocationId.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.ddefaultLocationId.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.ddefaultLocationId.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.ddefaultLocationId.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.ddefaultLocationId.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.ddefaultLocationId.add(543,1,&quot;L - L&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.ddefaultLocationId.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.ddefaultLocationId.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.ddefaultLocationId.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.ddefaultLocationId.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.ddefaultLocationId.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.ddefaultLocationId.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.ddefaultLocationId.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.ddefaultLocationId.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.ddefaultLocationId.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.ddefaultLocationId.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.ddefaultLocationId.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.ddefaultLocationId.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.ddefaultLocationId.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.ddefaultLocationId.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.ddefaultLocationId.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.ddefaultLocationId.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.ddefaultLocationId.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.ddefaultLocationId.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.ddefaultLocationId.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.ddefaultLocationId.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.ddefaultLocationId.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.ddefaultLocationId.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.ddefaultLocationId.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.ddefaultLocationId.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.ddefaultLocationId.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.ddefaultLocationId.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.ddefaultLocationId.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.ddefaultLocationId.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.ddefaultLocationId.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.ddefaultLocationId.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.ddefaultLocationId.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.ddefaultLocationId.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.ddefaultLocationId.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.ddefaultLocationId.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.ddefaultLocationId.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.ddefaultLocationId.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.ddefaultLocationId.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.ddefaultLocationId.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.ddefaultLocationId.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.ddefaultLocationId.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.ddefaultLocationId.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.ddefaultLocationId.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.ddefaultLocationId.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.ddefaultLocationId.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.ddefaultLocationId.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.ddefaultLocationId.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.ddefaultLocationId.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.ddefaultLocationId.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.ddefaultLocationId.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.ddefaultLocationId.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;defaultLocationId_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_0&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;defaultLocationIdTree&quot; , &quot;'&quot; , &quot;); 
if(lname) { tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink_0&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l&quot; , &quot;'&quot; , &quot;+id,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink_0&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l-1&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;defaultLocationIdArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ddefaultLocationId; 
  
  
  



					*預設值 
						讀者身份類別
					
					
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

						
				
			
		
		
			
		
		
			
				
					
				
			
		
		
			
		
		
			
			修改/存檔
			
			
			
		
		
		
		
			
				借閱證號碼
				1234
			
		
	
	

 





 
  
    
    
    讀者證號資料
  
 
 
   



	

 查無資料


	


新證卡



 


 






	
		 
			
				   
				   
				   讀者悠遊卡資料
			 
		
		
	  		 



	

 查無資料


	


新悠遊卡


	 


		 
	







 
  
   
   
   讀者身分類別
  
 
 
   

	
	
	
		
		
		
			館藏地
		
			讀者身份類別
	
	
	
	
	









	
	
		 
			 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dSelectTreeStructure = new dTree(&quot; , &quot;'&quot; , &quot;window.dSelectTreeStructure&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dSelectTreeStructure.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dSelectTreeStructure.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dSelectTreeStructure.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dSelectTreeStructure.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dSelectTreeStructure.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dSelectTreeStructure.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dSelectTreeStructure.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dSelectTreeStructure.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dSelectTreeStructure.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dSelectTreeStructure.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dSelectTreeStructure.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dSelectTreeStructure.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dSelectTreeStructure.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dSelectTreeStructure.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dSelectTreeStructure.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dSelectTreeStructure.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dSelectTreeStructure.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dSelectTreeStructure.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dSelectTreeStructure.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dSelectTreeStructure.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dSelectTreeStructure.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dSelectTreeStructure.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dSelectTreeStructure.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dSelectTreeStructure.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dSelectTreeStructure.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dSelectTreeStructure.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dSelectTreeStructure.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dSelectTreeStructure.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dSelectTreeStructure.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dSelectTreeStructure.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dSelectTreeStructure.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dSelectTreeStructure.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dSelectTreeStructure.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dSelectTreeStructure.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dSelectTreeStructure.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dSelectTreeStructure.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dSelectTreeStructure.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dSelectTreeStructure.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dSelectTreeStructure.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dSelectTreeStructure.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dSelectTreeStructure.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dSelectTreeStructure.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dSelectTreeStructure.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dSelectTreeStructure.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dSelectTreeStructure.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dSelectTreeStructure.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dSelectTreeStructure.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dSelectTreeStructure.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dSelectTreeStructure.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dSelectTreeStructure.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dSelectTreeStructure.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dSelectTreeStructure.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dSelectTreeStructure.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dSelectTreeStructure.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dSelectTreeStructure.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dSelectTreeStructure.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dSelectTreeStructure.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dSelectTreeStructure.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dSelectTreeStructure.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dSelectTreeStructure.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dSelectTreeStructure.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dSelectTreeStructure.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dSelectTreeStructure.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dSelectTreeStructure.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dSelectTreeStructure.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dSelectTreeStructure.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dSelectTreeStructure.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dSelectTreeStructure.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dSelectTreeStructure.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dSelectTreeStructure.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dSelectTreeStructure.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dSelectTreeStructure.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dSelectTreeStructure.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dSelectTreeStructure.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dSelectTreeStructure.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dSelectTreeStructure.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dSelectTreeStructure.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dSelectTreeStructure.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dSelectTreeStructure.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dSelectTreeStructure.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dSelectTreeStructure.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dSelectTreeStructure.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dSelectTreeStructure.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dSelectTreeStructure.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dSelectTreeStructure.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dSelectTreeStructure.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dSelectTreeStructure.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dSelectTreeStructure.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dSelectTreeStructure.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dSelectTreeStructure.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dSelectTreeStructure.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dSelectTreeStructure.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dSelectTreeStructure.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dSelectTreeStructure.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dSelectTreeStructure.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dSelectTreeStructure.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dSelectTreeStructure.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dSelectTreeStructure.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dSelectTreeStructure.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dSelectTreeStructure.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dSelectTreeStructure.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dSelectTreeStructure.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dSelectTreeStructure.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dSelectTreeStructure.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dSelectTreeStructure.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dSelectTreeStructure.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dSelectTreeStructure.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dSelectTreeStructure.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dSelectTreeStructure.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dSelectTreeStructure.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dSelectTreeStructure.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dSelectTreeStructure.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dSelectTreeStructure.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dSelectTreeStructure.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;SelectTreeStructure_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_1&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;SelectTreeStructureTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;SelectTreeStructureArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dSelectTreeStructure; 
  
  
  

   
    		
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

  	
	

	
	
		
			
				新增

		
	
	



 






 
  
   
   
   流通讀者狀態
  
 
 
 		  

window.suspendRight = function(data) {
	        var contextPath=$(&quot; , &quot;'&quot; , &quot;#contextPath&quot; , &quot;'&quot; , &quot;).text();
		    var printType=&quot;PREVIEW&quot;;
		    var code=data;
		    
		     var oForm = document.createElement(&quot;form&quot;);  
		     oForm.id=&quot;jasper&quot;;  
		     oForm.method=&quot;post&quot;;  
		     oForm.action=contextPath+&quot;struts/suspendRight.jsp&quot;;  
		     
		     var keys=[&quot; , &quot;'&quot; , &quot;codeX&quot; , &quot;'&quot; , &quot;];  
		     var values=[code];
		       
			 if (keys &amp;&amp; values &amp;&amp; (keys.length == values.length)){  
			    for (var i=0; i &lt; keys.length; i++){  
			        var oInput = document.createElement(&quot;input&quot;);  
			        oInput.type=&quot;hidden&quot;;  
			        oInput.name=keys[i];  
			        oInput.value=values[i];  
			        oForm.appendChild(oInput);  
			    }
			 }
			 if(&quot; , &quot;'&quot; , &quot;PREVIEW&quot; , &quot;'&quot; , &quot;==printType){
			     oForm.target=&quot;jasperTarget&quot;;
			     oForm.onSubmit=function(){openSpecfiyWindown(&quot;jasperWindown&quot;)};
		     }
		     document.body.appendChild(oForm);  
		     oForm.submit();   
}




(function($){$(document).ready(function(){$(&quot;#borrowedFlag&quot;).hide();if($(&quot;#isLanguage&quot;).html()==&quot;1&quot;){$(&quot;#borrowedFlag&quot;).show();}});})(jQuery);
 


var patronItemDetails = document.getElementById(&quot;PatronItemDetails&quot;);
var isShowPatronPhotoLoanDesk = document.getElementById(&quot;isShowPatronPhotoLoanDesk&quot;);
var reset = document.getElementById(&quot;reset&quot;);
if(isShowPatronPhotoLoanDesk){
	patronItemDetails.style.marginBottom = &quot;-49px&quot;;
    patronItemDetails.style.marginTop = &quot;-72px&quot;;
    reset.style.marginTop = &quot;-77px&quot;;
}


	.tdlwx{ font-weight:700; font-style:italic; font-family:Verdana, Geneva, sans-serif;white-space:nowrap; width:35px;}


	1

	/inspireapp/


	
	
		
			
				未選擇讀者資料
		
	
	

	

	
	    
	    
 
  
    罰款
  
  
 
 
  
 

	
	
		
	    
 
  
    Penalties
  
  
 
 
  
 

	



 
 
  
    預約可取
  
  
 
 
  
 


 
 
  
    逾期罰金估算
  
  
 
 
  
 




 
  
    讀者基本資訊
  
  
 
 
  
 




 
 







	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;patronEditForm&quot;);
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }
calendar_lockedDatetimeDatePicker = new Calendar();

	
calendar_lockedDatetimeDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_lockedDatetimeDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).lockedDatetimeDatePicker;
  var value = calendar_lockedDatetimeDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_equEndDateDatePicker = new Calendar();

	
calendar_equEndDateDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_equEndDateDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).equEndDateDatePicker;
  var value = calendar_equEndDateDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

tapestry.form.registerForm(&quot;classesForm&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;PenaltiesDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;SiteEquPenaltiesDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;HoldsListDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;estimatedPenaltiesArea&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;detailsDialog&quot; , &quot;'&quot; , &quot;);
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
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->




&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear/html[1]&quot;) or . = concat(&quot;



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

var calendar_lockedDatetimeDatePicker;
var calendar_equEndDateDatePicker;
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

 

	

	
		
		  
		    
	
		讀者資料維護
		
		
			
		        讀者基本資料
			
		
	

		  
		  
		  
		    
			
			
		  
		


	
	     


 


 
  
   
    
    
    一般細節
   
  
  
 
 
  
	
		
	
	
	
	
	
		$(document).keypress(function(e) {
			if (e.which == 13) {
				return false;
			}
		});
	
	
		(function($) {
			//日期選擇器
			function initDatePicker() {
		        var contextPath = trim($(&quot; , &quot;'&quot; , &quot;#contextPath&quot; , &quot;'&quot; , &quot;).text());
		        $(&quot; , &quot;'&quot; , &quot;.DatePickerID&quot; , &quot;'&quot; , &quot;).datepick({
		            dateFormat: &quot; , &quot;'&quot; , &quot;yy/mm/dd&quot; , &quot;'&quot; , &quot;,
		            showOn: &quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot;,
		            buttonImageOnly: true,
		            buttonImage: contextPath + &quot; , &quot;'&quot; , &quot;/js/datepick/DatePickerIcon.png&quot; , &quot;'&quot; , &quot;,
		            yearRange: &quot; , &quot;'&quot; , &quot;c-100:c+50&quot; , &quot;'&quot; , &quot;
		        });
		    }
		    // 在文檔準備好時初始化日期選擇器
		    $(document).ready(function() {
		        initDatePicker();
		    });
		 	// 使用MutationObserver監聽文檔內容的變化
		    var observer = new MutationObserver(function(mutations) {
		        mutations.forEach(function(mutation) {
		            if (mutation.type === &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
		                // 文檔內容發生變化時重新初始化日期選擇器
		                initDatePicker();
		            }
		        });
		    });
		    // 監聽整個文檔的變化
		    observer.observe(document, { childList: true, subtree: true });
		})(jQuery);
	
	
		(function($) {
			$(document).ready(function() {
				$(&quot;#dateToBeCheckedflag&quot;).hide();
				if ($(&quot;#isLanguage&quot;).html() == &quot;1&quot;) {
					$(&quot;#dateToBeCheckedflag&quot;).show();
				}
			});
		})(jQuery);
	
	

#loading-overlay { position: absolute; z-index: 2147483647; width: 1200px; height: 1500px; top: 0; left: 0; right: 0; bottom: -500; background-color: transparent; opacity: 0.7; }

	




















		 
	function blockScreen_0(e){
		var iDiv = window.document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);
		iDiv.id = &quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;; 
		var blockDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv_0&quot; , &quot;'&quot; , &quot;);
		if (blockDiv!=null) blockDiv.appendChild(iDiv);
	} 
	document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function() {
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButton&quot;), &quot;onclick&quot;, blockScreen_0);
		dojo.event.connect(&quot;before&quot;, dojo.byId(&quot;savePatronButtonWithQuest&quot;), &quot;onclick&quot;, blockScreen_0);
	}, false);





//&lt;![CDATA[

	 function deleteblock_0(){
		var parent = document.getElementById(&quot; , &quot;'&quot; , &quot;blockDiv_0&quot; , &quot;'&quot; , &quot;);
		var child = document.getElementById(&quot; , &quot;'&quot; , &quot;loading-overlay&quot; , &quot;'&quot; , &quot;);		
		if(child != null){
			parent.removeChild(child);
		}	
	}
	deleteblock_0();
 
//]]&gt;




	
		輸入欄位驗証錯誤提示行動電話電話號碼格式錯誤 
		1
		
			/inspireapp/
		
		
			   
		
		
		
		
			
						
				
					*姓名:
					

					暱稱:
					
				
				
					*帳號:
					
							
						 
							   1234  帳號修改
							
						

					* 身分證號:
					
				
				
					行動電話:
					







					讀者狀態:
					
有效讀者
離職離校
微波通訊
MATLAB程式設計
無線行動網路架構
電力電子學
硬體描述語言設計
電路板設計實務
PHP程式設計
通訊實習
手持式裝置設計與應用
進階數學
半導體元件及量測實務
科技英文
創意與專利1

				
				
					單位所系:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
所系單位_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree(&quot; , &quot;'&quot; , &quot;window.ddepartment&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(0,-1,&quot; , &quot;'&quot; , &quot;所系單位&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(441,0,&quot;_New Item&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;_New Item&quot; , &quot;'&quot; , &quot;, 441, true)&quot;); 
window.ddepartment.add(461,0,&quot;_New Item1234&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;_New Item1234&quot; , &quot;'&quot; , &quot;, 461, true)&quot;); 
window.ddepartment.add(121,0,&quot;\u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 121, true)&quot;); 
window.ddepartment.add(141,121,&quot;\u884C\u653F\u55AE\u4F4D\u5176\u4ED6\u5176\u4ED6\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u884C\\u653F\\u55AE\\u4F4D\\u5176\\u4ED6\\u5176\\u4ED6\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 141, true)&quot;); 
window.ddepartment.add(145,141,&quot;\u5B78\u6821\u884C\u653F&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u884C\\u653F&quot; , &quot;'&quot; , &quot;, 145, true)&quot;); 
window.ddepartment.add(167,145,&quot;\u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.ddepartment.add(168,145,&quot;\u5317\u6E2F\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 168, true)&quot;); 
window.ddepartment.add(122,145,&quot;\u5B89\u5357\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B89\\u5357\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 122, true)&quot;); 
window.ddepartment.add(123,145,&quot;\u6C34\u6E73\u5716\u66F8\u5206\u9928&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6C34\\u6E73\\u5716\\u66F8\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 123, true)&quot;); 
window.ddepartment.add(124,145,&quot;\u5B78\u6821\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 124, true)&quot;); 
window.ddepartment.add(125,145,&quot;\u5317\u6E2F\u5206\u90E8\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u5206\\u90E8\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 125, true)&quot;); 
window.ddepartment.add(126,145,&quot;\u5B78\u6821\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u6821\\u7814\\u7A76\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 126, true)&quot;); 
window.ddepartment.add(142,121,&quot;\u5B78\u8853\u55AE\u4F4D\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u8853\\u55AE\\u4F4D\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 142, true)&quot;); 
window.ddepartment.add(146,142,&quot;\u91AB\u5B78\u9662\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u9662\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 146, true)&quot;); 
window.ddepartment.add(127,146,&quot;\u91AB\u5B78\u7CFB\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7CFB\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 127, true)&quot;); 
window.ddepartment.add(401,127,&quot;\u91AB\u5B78\u7CFB\u4E00\u5E74A\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7CFB\\u4E00\\u5E74A\\u73ED&quot; , &quot;'&quot; , &quot;, 401, true)&quot;); 
window.ddepartment.add(128,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 128, true)&quot;); 
window.ddepartment.add(129,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 129, true)&quot;); 
window.ddepartment.add(130,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 130, true)&quot;); 
window.ddepartment.add(131,146,&quot;\u91AB\u5B78\u6AA2\u9A57\u751F\u7269\u6280\u8853\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u6AA2\\u9A57\\u751F\\u7269\\u6280\\u8853\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 131, true)&quot;); 
window.ddepartment.add(132,146,&quot;\u751F\u7269\u91AB\u5B78\u5F71\u50CF\u66A8\u653E\u5C04\u79D1\u5B78\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5F71\\u50CF\\u66A8\\u653E\\u5C04\\u79D1\\u5B78\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 132, true)&quot;); 
window.ddepartment.add(133,146,&quot;\u570B\u969B\u751F\u7269\u91AB\u5B78\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u570B\\u969B\\u751F\\u7269\\u91AB\\u5B78\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 133, true)&quot;); 
window.ddepartment.add(134,146,&quot;\u81E8\u5E8A\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u81E8\\u5E8A\\u91AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 134, true)&quot;); 
window.ddepartment.add(135,146,&quot;\u57FA\u790E\u91AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u57FA\\u790E\\u91AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 135, true)&quot;); 
window.ddepartment.add(136,146,&quot;\u514D\u75AB\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u514D\\u75AB\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 136, true)&quot;); 
window.ddepartment.add(137,146,&quot;\u764C\u75C7\u751F\u7269\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u751F\\u7269\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 137, true)&quot;); 
window.ddepartment.add(138,146,&quot;\u795E\u7D93\u79D1\u5B78\u8207\u8A8D\u77E5\u79D1\u5B78\u7814\u7A76\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u79D1\\u5B78\\u8207\\u8A8D\\u77E5\\u79D1\\u5B78\\u7814\\u7A76\\u6240&quot; , &quot;'&quot; , &quot;, 138, true)&quot;); 
window.ddepartment.add(139,146,&quot;\u751F\u7269\u91AB\u5B78\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 139, true)&quot;); 
window.ddepartment.add(169,146,&quot;\u764C\u75C7\u751F\u7269\u8207\u85E5\u7269\u7814\u767C\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u751F\\u7269\\u8207\\u85E5\\u7269\\u7814\\u767C\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.ddepartment.add(170,146,&quot;\u8001\u5316\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8001\\u5316\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.ddepartment.add(171,146,&quot;\u8F49\u8B6F\u91AB\u5B78\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8F49\\u8B6F\\u91AB\\u5B78\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.ddepartment.add(321,146,&quot;\u751F\u91AB\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u91AB\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 321, true)&quot;); 
window.ddepartment.add(147,142,&quot;\u4E2D\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 147, true)&quot;); 
window.ddepartment.add(421,147,&quot;New Item&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;New Item&quot; , &quot;'&quot; , &quot;, 421, true)&quot;); 
window.ddepartment.add(172,147,&quot;\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 172, true)&quot;); 
window.ddepartment.add(173,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u7532\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u7532\\u7D44&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.ddepartment.add(174,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u4E59\u7D44&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u4E59\\u7D44&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.ddepartment.add(175,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.ddepartment.add(176,147,&quot;\u5B78\u58EB\u5F8C\u4E2D\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B78\\u58EB\\u5F8C\\u4E2D\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.ddepartment.add(177,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.ddepartment.add(178,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 178, true)&quot;); 
window.ddepartment.add(140,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 140, true)&quot;); 
window.ddepartment.add(181,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.ddepartment.add(182,147,&quot;\u570B\u969B\u91DD\u7078\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u570B\\u969B\\u91DD\\u7078\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.ddepartment.add(183,147,&quot;\u4E2D\u7378\u91AB\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u7378\\u91AB\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.ddepartment.add(184,147,&quot;\u4E2D\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 184, true)&quot;); 
window.ddepartment.add(185,147,&quot;\u4E2D\u897F\u91AB\u7D50\u5408\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u897F\\u91AB\\u7D50\\u5408\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 185, true)&quot;); 
window.ddepartment.add(186,147,&quot;\u91DD\u7078\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91DD\\u7078\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 186, true)&quot;); 
window.ddepartment.add(187,147,&quot;\u4E2D\u570B\u85E5\u5B78\u66A8\u4E2D\u85E5\u8CC7\u6E90\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u85E5\\u5B78\\u66A8\\u4E2D\\u85E5\\u8CC7\\u6E90\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 187, true)&quot;); 
window.ddepartment.add(148,142,&quot;\u85E5\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 148, true)&quot;); 
window.ddepartment.add(179,148,&quot;\u85E5\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.ddepartment.add(180,148,&quot;\u85E5\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.ddepartment.add(201,148,&quot;\u85E5\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 201, true)&quot;); 
window.ddepartment.add(202,148,&quot;\u751F\u6280\u88FD\u85E5\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u6280\\u88FD\\u85E5\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 202, true)&quot;); 
window.ddepartment.add(149,142,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 149, true)&quot;); 
window.ddepartment.add(203,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 203, true)&quot;); 
window.ddepartment.add(204,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 204, true)&quot;); 
window.ddepartment.add(205,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 205, true)&quot;); 
window.ddepartment.add(206,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u9662\u5927\u4E00\u4E0D\u5206\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u9662\\u5927\\u4E00\\u4E0D\\u5206\\u7CFB&quot; , &quot;'&quot; , &quot;, 206, true)&quot;); 
window.ddepartment.add(207,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 207, true)&quot;); 
window.ddepartment.add(208,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 208, true)&quot;); 
window.ddepartment.add(209,149,&quot;\u516C\u5171\u885B\u751F\u570B\u969B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u570B\\u969B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 209, true)&quot;); 
window.ddepartment.add(210,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 210, true)&quot;); 
window.ddepartment.add(211,149,&quot;\u8077\u696D\u5B89\u5168\u8207\u885B\u751F\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8077\\u696D\\u5B89\\u5168\\u8207\\u885B\\u751F\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.ddepartment.add(212,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 212, true)&quot;); 
window.ddepartment.add(213,149,&quot;\u91AB\u52D9\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u52D9\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 213, true)&quot;); 
window.ddepartment.add(214,149,&quot;\u5065\u5EB7\u98A8\u96AA\u7BA1\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u98A8\\u96AA\\u7BA1\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 214, true)&quot;); 
window.ddepartment.add(215,149,&quot;\u751F\u7269\u7D71\u8A08\u7814\u7A76\u6240\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u7D71\\u8A08\\u7814\\u7A76\\u6240\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 215, true)&quot;); 
window.ddepartment.add(216,149,&quot;\u516C\u5171\u885B\u751F\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u516C\\u5171\\u885B\\u751F\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 216, true)&quot;); 
window.ddepartment.add(381,149,&quot;\u55AE\u4F4D\u7CFB\u6240\u532F\u5165&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u55AE\\u4F4D\\u7CFB\\u6240\\u532F\\u5165&quot; , &quot;'&quot; , &quot;, 381, true)&quot;); 
window.ddepartment.add(191,149,&quot;\u91AB\u5B78\u5DE5\u7A0B\u8207\u5FA9\u5065\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u5DE5\\u7A0B\\u8207\\u5FA9\\u5065\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 191, true)&quot;); 
window.ddepartment.add(245,149,&quot;\u751F\u7269\u91AB\u5B78\u5DE5\u7A0B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u91AB\\u5B78\\u5DE5\\u7A0B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.ddepartment.add(150,142,&quot;\u5065\u5EB7\u7167\u8B77\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u7167\\u8B77\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 150, true)&quot;); 
window.ddepartment.add(217,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.ddepartment.add(218,150,&quot;\u8B77\u7406\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.ddepartment.add(219,150,&quot;\u904B\u52D5\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u904B\\u52D5\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.ddepartment.add(220,150,&quot;\u53E3\u8154\u885B\u751F\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53E3\\u8154\\u885B\\u751F\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.ddepartment.add(221,150,&quot;\u4E8C\u5E74\u5236\u8B77\u7406\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E8C\\u5E74\\u5236\\u8B77\\u7406\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.ddepartment.add(188,150,&quot;\u4E8C\u5E74\u5236\u547C\u5438\u6CBB\u7642\u5B78\u7CFB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E8C\\u5E74\\u5236\\u547C\\u5438\\u6CBB\\u7642\\u5B78\\u7CFB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 188, true)&quot;); 
window.ddepartment.add(189,150,&quot;\u7269\u7406\u6CBB\u7642\u5B78\u7CFB\u5FA9\u5065\u79D1\u5B78\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7269\\u7406\\u6CBB\\u7642\\u5B78\\u7CFB\\u5FA9\\u5065\\u79D1\\u5B78\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 189, true)&quot;); 
window.ddepartment.add(190,150,&quot;\u8B77\u7406\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 190, true)&quot;); 
window.ddepartment.add(361,150,&quot;\u8B77\u7406\u5B78\u7CFB\u8DE8\u9818\u57DF\u9577\u671F\u7167\u8B77\u78A9\u58EB\u5728\u8077\u5C08\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u5B78\\u7CFB\\u8DE8\\u9818\\u57DF\\u9577\\u671F\\u7167\\u8B77\\u78A9\\u58EB\\u5728\\u8077\\u5C08\\u73ED&quot; , &quot;'&quot; , &quot;, 361, true)&quot;); 
window.ddepartment.add(192,150,&quot;\u5065\u5EB7\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5065\\u5EB7\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 192, true)&quot;); 
window.ddepartment.add(151,142,&quot;\u751F\u6280\u88FD\u85E5\u66A8\u98DF\u54C1\u79D1\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u6280\\u88FD\\u85E5\\u66A8\\u98DF\\u54C1\\u79D1\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 151, true)&quot;); 
window.ddepartment.add(193,151,&quot;\u71DF\u990A\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 193, true)&quot;); 
window.ddepartment.add(194,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 194, true)&quot;); 
window.ddepartment.add(195,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 195, true)&quot;); 
window.ddepartment.add(196,151,&quot;\u71DF\u990A\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 196, true)&quot;); 
window.ddepartment.add(197,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 197, true)&quot;); 
window.ddepartment.add(198,151,&quot;\u85E5\u7528\u5316\u599D\u54C1\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u7528\\u5316\\u599D\\u54C1\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 198, true)&quot;); 
window.ddepartment.add(199,151,&quot;\u88FD\u85E5\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u88FD\\u85E5\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 199, true)&quot;); 
window.ddepartment.add(200,151,&quot;\u98DF\u54C1\u66A8\u85E5\u7269\u5B89\u5168\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u98DF\\u54C1\\u66A8\\u85E5\\u7269\\u5B89\\u5168\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 200, true)&quot;); 
window.ddepartment.add(241,151,&quot;\u71DF\u990A\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u71DF\\u990A\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 241, true)&quot;); 
window.ddepartment.add(242,151,&quot;\u751F\u7269\u79D1\u6280\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 242, true)&quot;); 
window.ddepartment.add(243,151,&quot;\u65B0\u85E5\u958B\u767C\u7814\u7A76\u6240\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u65B0\\u85E5\\u958B\\u767C\\u7814\\u7A76\\u6240\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 243, true)&quot;); 
window.ddepartment.add(244,151,&quot;\u751F\u7269\u79D1\u6280\u7522\u696D\u535A\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u751F\\u7269\\u79D1\\u6280\\u7522\\u696D\\u535A\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.ddepartment.add(152,142,&quot;\u4EBA\u6587\u8207\u79D1\u6280\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4EBA\\u6587\\u8207\\u79D1\\u6280\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 152, true)&quot;); 
window.ddepartment.add(322,152,&quot;\u79D1\u6280\u6CD5\u5F8B\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B\u5176\u4ED6&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u79D1\\u6280\\u6CD5\\u5F8B\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B\\u5176\\u4ED6&quot; , &quot;'&quot; , &quot;, 322, true)&quot;); 
window.ddepartment.add(362,152,&quot;\u79D1\u6280\u7BA1\u7406\u78A9\u58EB\u5B78\u4F4D\u5B78\u7A0B&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u79D1\\u6280\\u7BA1\\u7406\\u78A9\\u58EB\\u5B78\\u4F4D\\u5B78\\u7A0B&quot; , &quot;'&quot; , &quot;, 362, true)&quot;); 
window.ddepartment.add(153,142,&quot;\u7259\u91AB\u5B78\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u9662&quot; , &quot;'&quot; , &quot;, 153, true)&quot;); 
window.ddepartment.add(246,153,&quot;\u7259\u91AB\u5B78\u7CFB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.ddepartment.add(247,153,&quot;\u7259\u91AB\u5B78\u7CFB\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 247, true)&quot;); 
window.ddepartment.add(363,153,&quot;\u7259\u91AB\u5B78\u7CFB\u53E3\u8154\u91AB\u5B78\u7522\u696D\u78A9\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u53E3\\u8154\\u91AB\\u5B78\\u7522\\u696D\\u78A9\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.ddepartment.add(323,153,&quot;\u7259\u91AB\u5B78\u7CFB\u535A\u58EB\u73ED&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u5B78\\u7CFB\\u535A\\u58EB\\u73ED&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.ddepartment.add(154,142,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 154, true)&quot;); 
window.ddepartment.add(248,154,&quot;\u901A\u8B58\u6559\u80B2\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u901A\\u8B58\\u6559\\u80B2\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.ddepartment.add(143,121,&quot;\u9644\u8A2D\u6A5F\u69CB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u8A2D\\u6A5F\\u69CB&quot; , &quot;'&quot; , &quot;, 143, true)&quot;); 
window.ddepartment.add(222,143,&quot;\u4E2D\u570B\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u9644\\u91AB&quot; , &quot;'&quot; , &quot;, 222, true)&quot;); 
window.ddepartment.add(223,222,&quot;\u9644\u91AB\u7814\u7A76\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB\\u7814\\u7A76\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 223, true)&quot;); 
window.ddepartment.add(224,222,&quot;\u5167\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5167\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 224, true)&quot;); 
window.ddepartment.add(225,222,&quot;\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5916\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 225, true)&quot;); 
window.ddepartment.add(226,222,&quot;\u795E\u7D93\u5916\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u5916\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 226, true)&quot;); 
window.ddepartment.add(249,222,&quot;\u9AA8\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9AA8\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 249, true)&quot;); 
window.ddepartment.add(250,222,&quot;\u6CCC\u5C3F\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6CCC\\u5C3F\\u90E8&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.ddepartment.add(251,222,&quot;\u5A66\u7522\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5A66\\u7522\\u90E8&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.ddepartment.add(227,222,&quot;\u795E\u7D93\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u90E8&quot; , &quot;'&quot; , &quot;, 227, true)&quot;); 
window.ddepartment.add(228,222,&quot;\u8033\u9F3B\u5589\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8033\\u9F3B\\u5589\\u90E8&quot; , &quot;'&quot; , &quot;, 228, true)&quot;); 
window.ddepartment.add(229,222,&quot;\u76AE\u819A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u76AE\\u819A\\u79D1&quot; , &quot;'&quot; , &quot;, 229, true)&quot;); 
window.ddepartment.add(230,222,&quot;\u7259\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7259\\u91AB\\u90E8&quot; , &quot;'&quot; , &quot;, 230, true)&quot;); 
window.ddepartment.add(231,222,&quot;\u7CBE\u795E\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u7CBE\\u795E\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 231, true)&quot;); 
window.ddepartment.add(232,222,&quot;\u5FA9\u5065\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5FA9\\u5065\\u90E8&quot; , &quot;'&quot; , &quot;, 232, true)&quot;); 
window.ddepartment.add(233,222,&quot;\u9EBB\u9189\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9EBB\\u9189\\u90E8&quot; , &quot;'&quot; , &quot;, 233, true)&quot;); 
window.ddepartment.add(235,222,&quot;\u81E8\u5E8A\u71DF\u990A\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u81E8\\u5E8A\\u71DF\\u990A\\u79D1&quot; , &quot;'&quot; , &quot;, 235, true)&quot;); 
window.ddepartment.add(234,222,&quot;\u4E2D\u91AB\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u91AB\\u90E8&quot; , &quot;'&quot; , &quot;, 234, true)&quot;); 
window.ddepartment.add(252,222,&quot;\u4E2D\u570B\u9644\u91AB\u884C\u653F\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u570B\\u9644\\u91AB\\u884C\\u653F\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.ddepartment.add(253,222,&quot;\u793E\u6703\u5DE5\u4F5C\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u793E\\u6703\\u5DE5\\u4F5C\\u5BA4&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.ddepartment.add(254,222,&quot;\u773C\u79D1\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u773C\\u79D1\\u90E8&quot; , &quot;'&quot; , &quot;, 254, true)&quot;); 
window.ddepartment.add(255,222,&quot;\u5152\u7AE5\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5152\\u7AE5\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 255, true)&quot;); 
window.ddepartment.add(256,222,&quot;\u75C5\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u75C5\\u7406\\u90E8&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.ddepartment.add(257,222,&quot;\u57FA\u56E0\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u57FA\\u56E0\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.ddepartment.add(258,222,&quot;\u9810\u9632\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9810\\u9632\\u91AB\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.ddepartment.add(259,222,&quot;\u91AB\u5B78\u7814\u7A76\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u7814\\u7A76\\u90E8&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.ddepartment.add(260,222,&quot;\u6559\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6559\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 260, true)&quot;); 
window.ddepartment.add(261,222,&quot;\u6025\u75C7\u66A8\u5916\u50B7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6025\\u75C7\\u66A8\\u5916\\u50B7\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.ddepartment.add(262,222,&quot;\u8B77\u7406\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8B77\\u7406\\u90E8&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.ddepartment.add(263,222,&quot;\u85E5\u5291\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u85E5\\u5291\\u90E8&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.ddepartment.add(264,222,&quot;\u91AB\u5B78\u5F71\u50CF\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u5B78\\u5F71\\u50CF\\u90E8&quot; , &quot;'&quot; , &quot;, 264, true)&quot;); 
window.ddepartment.add(265,222,&quot;\u6AA2\u9A57\u91AB\u5B78\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6AA2\\u9A57\\u91AB\\u5B78\\u90E8&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.ddepartment.add(266,222,&quot;\u6838\u5B50\u91AB\u5B78\u79D1&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6838\\u5B50\\u91AB\\u5B78\\u79D1&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.ddepartment.add(267,222,&quot;\u795E\u7D93\u7CBE\u795E\u91AB\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u795E\\u7D93\\u7CBE\\u795E\\u91AB\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.ddepartment.add(268,222,&quot;\u91AB\u7642\u54C1\u8CEA\u90E8&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u91AB\\u7642\\u54C1\\u8CEA\\u90E8&quot; , &quot;'&quot; , &quot;, 268, true)&quot;); 
window.ddepartment.add(269,222,&quot;\u764C\u75C7\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u764C\\u75C7\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 269, true)&quot;); 
window.ddepartment.add(155,143,&quot;\u9644\u91AB-\u5317\u6E2F\u9644\u91AB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5317\\u6E2F\\u9644\\u91AB&quot; , &quot;'&quot; , &quot;, 155, true)&quot;); 
window.ddepartment.add(270,155,&quot;\u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.ddepartment.add(156,143,&quot;\u9644\u91AB-\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 156, true)&quot;); 
window.ddepartment.add(271,156,&quot;\u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.ddepartment.add(157,143,&quot;\u9644\u91AB-\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4&quot; , &quot;'&quot; , &quot;, 157, true)&quot;); 
window.ddepartment.add(272,157,&quot;\u8C50\u539F\u91AB\u52D9\u5BA4&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8C50\\u539F\\u91AB\\u52D9\\u5BA4&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.ddepartment.add(158,143,&quot;\u9644\u91AB-\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 158, true)&quot;); 
window.ddepartment.add(273,158,&quot;\u53F0\u4E2D\u6771\u5340\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u4E2D\\u6771\\u5340\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.ddepartment.add(159,143,&quot;\u9644\u91AB-\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 159, true)&quot;); 
window.ddepartment.add(274,159,&quot;\u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.ddepartment.add(160,143,&quot;\u9644\u91AB-\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 160, true)&quot;); 
window.ddepartment.add(275,160,&quot;\u4E2D\u76E3\u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76E3\\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.ddepartment.add(301,143,&quot;\u9644\u91AB-\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240&quot; , &quot;'&quot; , &quot;, 301, true)&quot;); 
window.ddepartment.add(302,301,&quot;\u4E2D\u79D1\u5712\u5340\u54E1\u5DE5\u8A3A\u6240&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u79D1\\u5712\\u5340\\u54E1\\u5DE5\\u8A3A\\u6240&quot; , &quot;'&quot; , &quot;, 302, true)&quot;); 
window.ddepartment.add(161,143,&quot;\u9644\u91AB-\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u8349\\u5C6F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 161, true)&quot;); 
window.ddepartment.add(276,161,&quot;\u8349\u5C6F\u5206\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u8349\\u5C6F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.ddepartment.add(162,143,&quot;\u9644\u91AB-\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 162, true)&quot;); 
window.ddepartment.add(277,162,&quot;\u967D\u5149\u7CBE\u795E\u79D1\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u967D\\u5149\\u7CBE\\u795E\\u79D1\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 277, true)&quot;); 
window.ddepartment.add(163,143,&quot;\u9644\u91AB-\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 163, true)&quot;); 
window.ddepartment.add(278,163,&quot;\u5730\u5229\u6751\u9580\u8A3A\u4E2D\u5FC3&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5730\\u5229\\u6751\\u9580\\u8A3A\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 278, true)&quot;); 
window.ddepartment.add(164,143,&quot;\u9644\u91AB-\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u91AB-\\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 164, true)&quot;); 
window.ddepartment.add(279,164,&quot;\u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 279, true)&quot;); 
window.ddepartment.add(144,121,&quot;\u6821\u5916\u55AE\u4F4D&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916\\u55AE\\u4F4D&quot; , &quot;'&quot; , &quot;, 144, true)&quot;); 
window.ddepartment.add(165,144,&quot;\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u9928\\u969B\\u5408\\u4F5C&quot; , &quot;'&quot; , &quot;, 165, true)&quot;); 
window.ddepartment.add(236,165,&quot;NDDS\u9928\u969B\u5408\u4F5C&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;NDDS\\u9928\\u969B\\u5408\\u4F5C&quot; , &quot;'&quot; , &quot;, 236, true)&quot;); 
window.ddepartment.add(237,165,&quot;\u4E92\u501F\u5354\u8B70\u806F\u76DF&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E92\\u501F\\u5354\\u8B70\\u806F\\u76DF&quot; , &quot;'&quot; , &quot;, 237, true)&quot;); 
window.ddepartment.add(238,165,&quot;\u4E2D\u76DF-\u5927\u8449\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5927\\u8449\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 238, true)&quot;); 
window.ddepartment.add(239,165,&quot;\u4E2D\u76DF-\u4E2D\u5C71\u91AB\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u5C71\\u91AB\\u5927&quot; , &quot;'&quot; , &quot;, 239, true)&quot;); 
window.ddepartment.add(240,165,&quot;\u4E2D\u76DF-\u4E2D\u81FA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u81FA\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 240, true)&quot;); 
window.ddepartment.add(281,165,&quot;\u4E2D\u76DF-\u4E2D\u8208\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E2D\\u8208\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 281, true)&quot;); 
window.ddepartment.add(282,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u6559\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u6559\\u5927&quot; , &quot;'&quot; , &quot;, 282, true)&quot;); 
window.ddepartment.add(283,165,&quot;\u4E2D\u76DF-\u5F18\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5F18\\u5149\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.ddepartment.add(284,165,&quot;\u4E2D\u76DF-\u4E9E\u6D32\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4E9E\\u6D32\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.ddepartment.add(285,165,&quot;\u4E2D\u76DF-\u6771\u6D77\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u6771\\u6D77\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.ddepartment.add(286,165,&quot;\u4E2D\u76DF-\u5EFA\u570B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5EFA\\u570B\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.ddepartment.add(287,165,&quot;\u4E2D\u76DF-\u66A8\u5357\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u66A8\\u5357\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 287, true)&quot;); 
window.ddepartment.add(288,165,&quot;\u4E2D\u76DF-\u9022\u7532\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u9022\\u7532\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 288, true)&quot;); 
window.ddepartment.add(289,165,&quot;\u4E2D\u76DF-\u671D\u967D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u671D\\u967D\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 289, true)&quot;); 
window.ddepartment.add(290,165,&quot;\u4E2D\u76DF-\u52E4\u76CA\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u52E4\\u76CA\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 290, true)&quot;); 
window.ddepartment.add(291,165,&quot;\u4E2D\u76DF-\u5F70\u5316\u5E2B\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5F70\\u5316\\u5E2B\\u5927&quot; , &quot;'&quot; , &quot;, 291, true)&quot;); 
window.ddepartment.add(292,165,&quot;\u4E2D\u76DF-\u975C\u5B9C\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u975C\\u5B9C\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 292, true)&quot;); 
window.ddepartment.add(293,165,&quot;\u4E2D\u76DF-\u5DBA\u6771\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5DBA\\u6771\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 293, true)&quot;); 
window.ddepartment.add(294,165,&quot;\u4E2D\u76DF-\u53F0\u4E2D\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u53F0\\u4E2D\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 294, true)&quot;); 
window.ddepartment.add(295,165,&quot;\u4E2D\u76DF-\u806F\u5408\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u806F\\u5408\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 295, true)&quot;); 
window.ddepartment.add(296,165,&quot;\u4E2D\u76DF-\u660E\u9053\u5927\u5B78&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u660E\\u9053\\u5927\\u5B78&quot; , &quot;'&quot; , &quot;, 296, true)&quot;); 
window.ddepartment.add(297,165,&quot;\u4E2D\u76DF-\u5357\u958B\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u5357\\u958B\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 297, true)&quot;); 
window.ddepartment.add(298,165,&quot;\u4E2D\u76DF-\u4FEE\u5E73\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u4FEE\\u5E73\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 298, true)&quot;); 
window.ddepartment.add(299,165,&quot;\u4E2D\u76DF-\u80B2\u9054\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u80B2\\u9054\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 299, true)&quot;); 
window.ddepartment.add(300,165,&quot;\u4E2D\u76DF-\u50D1\u5149\u79D1\u5927&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E2D\\u76DF-\\u50D1\\u5149\\u79D1\\u5927&quot; , &quot;'&quot; , &quot;, 300, true)&quot;); 
window.ddepartment.add(166,144,&quot;\u6821\u5916&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916&quot; , &quot;'&quot; , &quot;, 166, true)&quot;); 
window.ddepartment.add(280,166,&quot;\u6821\u5916\u4EBA\u58EB&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;\\u6821\\u5916\\u4EBA\\u58EB&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.ddepartment.add(481,0,&quot;test123&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test123&quot; , &quot;'&quot; , &quot;, 481, true)&quot;); 
window.ddepartment.add(501,0,&quot;test234&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test234&quot; , &quot;'&quot; , &quot;, 501, true)&quot;); 
window.ddepartment.add(502,0,&quot;test777&quot;, &quot;javascript:window.ddepartment.selectElement(&quot; , &quot;'&quot; , &quot;test777&quot; , &quot;'&quot; , &quot;, 502, true)&quot;); 
window.ddepartment.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;department_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;departmentTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;departmentArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ddepartment; 
  
  
  










						
				
				
					代借帳號:
						
					
				
				
					流通停權:
					
					
					 電子郵件信箱(發送E-mail通知):
					
					
				
				
					
						OPAC登入停權:
					
					
						
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
				
				
					
						場地設備停權迄日:
					
						 
						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23

						

01
02
03
04
05
06
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59

					
					
					流通臺提示訊息:
					請修改密碼,帳密不要相同

					
						
					
				
				
					館員內部註記:
					
				
			
		
		
			
					
						
						
				
					新增日期:
					2018/02/14 14:38:51

					出生日期:
					
							
					
				
				
				
				
				
															
				
				讀者勾選同意時間:
					
													
				
				
				
					檢查日期:
					
					
				
				
					
					*可開始借閱日:
						

						*一般權限到期:
						
					
					
				
				
					 其他權限
					
				

				
				
					恢復預約權限日:
					

					較高權限到期:
					
				
				

				
				
					最高權限起自:
					

					最高權限到期:
					
				
				
				
					*預設值 
						所屬圖書館:
					 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.ddefaultLocationId = new dTree(&quot; , &quot;'&quot; , &quot;window.ddefaultLocationId&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddefaultLocationId.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.ddefaultLocationId.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.ddefaultLocationId.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.ddefaultLocationId.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.ddefaultLocationId.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.ddefaultLocationId.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.ddefaultLocationId.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.ddefaultLocationId.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.ddefaultLocationId.add(643,1,&quot;av - av&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.ddefaultLocationId.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.ddefaultLocationId.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.ddefaultLocationId.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.ddefaultLocationId.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.ddefaultLocationId.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.ddefaultLocationId.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.ddefaultLocationId.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.ddefaultLocationId.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.ddefaultLocationId.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.ddefaultLocationId.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.ddefaultLocationId.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.ddefaultLocationId.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.ddefaultLocationId.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.ddefaultLocationId.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.ddefaultLocationId.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.ddefaultLocationId.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.ddefaultLocationId.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.ddefaultLocationId.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.ddefaultLocationId.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.ddefaultLocationId.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.ddefaultLocationId.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.ddefaultLocationId.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.ddefaultLocationId.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.ddefaultLocationId.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.ddefaultLocationId.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.ddefaultLocationId.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.ddefaultLocationId.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.ddefaultLocationId.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.ddefaultLocationId.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.ddefaultLocationId.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.ddefaultLocationId.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.ddefaultLocationId.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.ddefaultLocationId.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.ddefaultLocationId.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.ddefaultLocationId.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.ddefaultLocationId.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.ddefaultLocationId.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.ddefaultLocationId.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.ddefaultLocationId.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.ddefaultLocationId.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.ddefaultLocationId.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.ddefaultLocationId.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.ddefaultLocationId.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.ddefaultLocationId.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.ddefaultLocationId.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.ddefaultLocationId.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.ddefaultLocationId.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.ddefaultLocationId.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.ddefaultLocationId.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.ddefaultLocationId.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.ddefaultLocationId.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.ddefaultLocationId.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.ddefaultLocationId.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.ddefaultLocationId.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.ddefaultLocationId.add(543,1,&quot;L - L&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.ddefaultLocationId.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.ddefaultLocationId.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.ddefaultLocationId.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.ddefaultLocationId.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.ddefaultLocationId.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.ddefaultLocationId.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.ddefaultLocationId.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.ddefaultLocationId.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.ddefaultLocationId.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.ddefaultLocationId.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.ddefaultLocationId.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.ddefaultLocationId.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.ddefaultLocationId.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.ddefaultLocationId.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.ddefaultLocationId.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.ddefaultLocationId.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.ddefaultLocationId.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.ddefaultLocationId.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.ddefaultLocationId.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.ddefaultLocationId.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.ddefaultLocationId.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.ddefaultLocationId.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.ddefaultLocationId.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.ddefaultLocationId.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.ddefaultLocationId.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.ddefaultLocationId.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.ddefaultLocationId.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.ddefaultLocationId.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.ddefaultLocationId.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.ddefaultLocationId.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.ddefaultLocationId.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.ddefaultLocationId.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.ddefaultLocationId.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.ddefaultLocationId.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.ddefaultLocationId.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.ddefaultLocationId.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.ddefaultLocationId.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.ddefaultLocationId.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.ddefaultLocationId.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.ddefaultLocationId.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.ddefaultLocationId.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.ddefaultLocationId.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.ddefaultLocationId.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.ddefaultLocationId.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.ddefaultLocationId.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.ddefaultLocationId.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.ddefaultLocationId.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.ddefaultLocationId.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.ddefaultLocationId.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.ddefaultLocationId.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.ddefaultLocationId.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;defaultLocationId_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_0&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;defaultLocationIdTree&quot; , &quot;'&quot; , &quot;); 
if(lname) { tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink_0&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l&quot; , &quot;'&quot; , &quot;+id,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
else { 
tapestry.linkOnClick(document.getElementById(&quot; , &quot;'&quot; , &quot;selectLink_0&quot; , &quot;'&quot; , &quot;).href+&quot; , &quot;'&quot; , &quot;?sp=l-1&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;selectLink&quot; , &quot;'&quot; , &quot;, false); 
 } 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;defaultLocationIdArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.ddefaultLocationId; 
  
  
  



					*預設值 
						讀者身份類別
					
					
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

						
				
			
		
		
			
		
		
			
				
					
				
			
		
		
			
		
		
			
			修改/存檔
			
			
			
		
		
		
		
			
				借閱證號碼
				1234
			
		
	
	

 





 
  
    
    
    讀者證號資料
  
 
 
   



	

 查無資料


	


新證卡



 


 






	
		 
			
				   
				   
				   讀者悠遊卡資料
			 
		
		
	  		 



	

 查無資料


	


新悠遊卡


	 


		 
	







 
  
   
   
   讀者身分類別
  
 
 
   

	
	
	
		
		
		
			館藏地
		
			讀者身份類別
	
	
	
	
	









	
	
		 
			 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dSelectTreeStructure = new dTree(&quot; , &quot;'&quot; , &quot;window.dSelectTreeStructure&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dSelectTreeStructure.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dSelectTreeStructure.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dSelectTreeStructure.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dSelectTreeStructure.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dSelectTreeStructure.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dSelectTreeStructure.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dSelectTreeStructure.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dSelectTreeStructure.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dSelectTreeStructure.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dSelectTreeStructure.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dSelectTreeStructure.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dSelectTreeStructure.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dSelectTreeStructure.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dSelectTreeStructure.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dSelectTreeStructure.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dSelectTreeStructure.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dSelectTreeStructure.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dSelectTreeStructure.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dSelectTreeStructure.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dSelectTreeStructure.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dSelectTreeStructure.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dSelectTreeStructure.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dSelectTreeStructure.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dSelectTreeStructure.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dSelectTreeStructure.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dSelectTreeStructure.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dSelectTreeStructure.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dSelectTreeStructure.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dSelectTreeStructure.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dSelectTreeStructure.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dSelectTreeStructure.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dSelectTreeStructure.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dSelectTreeStructure.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dSelectTreeStructure.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dSelectTreeStructure.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dSelectTreeStructure.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dSelectTreeStructure.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dSelectTreeStructure.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dSelectTreeStructure.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dSelectTreeStructure.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dSelectTreeStructure.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dSelectTreeStructure.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dSelectTreeStructure.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dSelectTreeStructure.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dSelectTreeStructure.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dSelectTreeStructure.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dSelectTreeStructure.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dSelectTreeStructure.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dSelectTreeStructure.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dSelectTreeStructure.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dSelectTreeStructure.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dSelectTreeStructure.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dSelectTreeStructure.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dSelectTreeStructure.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dSelectTreeStructure.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dSelectTreeStructure.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dSelectTreeStructure.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dSelectTreeStructure.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dSelectTreeStructure.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dSelectTreeStructure.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dSelectTreeStructure.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dSelectTreeStructure.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dSelectTreeStructure.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dSelectTreeStructure.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dSelectTreeStructure.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dSelectTreeStructure.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dSelectTreeStructure.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dSelectTreeStructure.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dSelectTreeStructure.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dSelectTreeStructure.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dSelectTreeStructure.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dSelectTreeStructure.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dSelectTreeStructure.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dSelectTreeStructure.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dSelectTreeStructure.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dSelectTreeStructure.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dSelectTreeStructure.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dSelectTreeStructure.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dSelectTreeStructure.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dSelectTreeStructure.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dSelectTreeStructure.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dSelectTreeStructure.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dSelectTreeStructure.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dSelectTreeStructure.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dSelectTreeStructure.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dSelectTreeStructure.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dSelectTreeStructure.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dSelectTreeStructure.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dSelectTreeStructure.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dSelectTreeStructure.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dSelectTreeStructure.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dSelectTreeStructure.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dSelectTreeStructure.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dSelectTreeStructure.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dSelectTreeStructure.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dSelectTreeStructure.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dSelectTreeStructure.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dSelectTreeStructure.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dSelectTreeStructure.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dSelectTreeStructure.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dSelectTreeStructure.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dSelectTreeStructure.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dSelectTreeStructure.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dSelectTreeStructure.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dSelectTreeStructure.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dSelectTreeStructure.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dSelectTreeStructure.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dSelectTreeStructure.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dSelectTreeStructure.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dSelectTreeStructure.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dSelectTreeStructure.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dSelectTreeStructure.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dSelectTreeStructure.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dSelectTreeStructure.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dSelectTreeStructure.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;SelectTreeStructure_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName_1&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;SelectTreeStructureTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;SelectTreeStructureArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dSelectTreeStructure; 
  
  
  

   
    		
01大學生一年級
02大學生二年級
03大學生三年級
04大學生四年級
05大學生五年級
06大學生六年級
07大學生七年級
08大學生八年級
09大學畢業生
10延畢生
111
11碩士班一年級
12碩士班二年級
13碩士班三年級
14碩士班四年級
15博士班一年級
16博士班二年級
17博士班三年級
18博士班四年級
19博士班五年級
20博士班六年級
21博士班七年級
22預研生
23碩博休學生
24專任教師
25兼任教師
26學校職員
27學校研究助理
28交換師生
29研究計劃
30訪問學人
31館內使用
32主治醫師
33住院醫師
34代訓醫師
35兼任醫師
36實習醫學生
37行政人員
38研究員
39醫檢師
40藥師
41護理師
42技術師
43治療師
44社工師
45營養師
46其他人員
47醫院研究助理
48附醫實習學生(不借書)
49附醫實習學生(可借書)
50推廣教育學員
51館際合作
52中區聯盟
53彰雲嘉聯盟
54附醫護生
55校友
56退休人員
57尊爵會員
58個人會員
59員眷
60電子資源權限(校外)
61無權限畢業生
62當年度畢業生
ererrrr
ERM同步
ERM測試身分
Nick
qkwmvud
TEST
日間部四年制學生

  	
	

	
	
		
			
				新增

		
	
	



 






 
  
   
   
   流通讀者狀態
  
 
 
 		  

window.suspendRight = function(data) {
	        var contextPath=$(&quot; , &quot;'&quot; , &quot;#contextPath&quot; , &quot;'&quot; , &quot;).text();
		    var printType=&quot;PREVIEW&quot;;
		    var code=data;
		    
		     var oForm = document.createElement(&quot;form&quot;);  
		     oForm.id=&quot;jasper&quot;;  
		     oForm.method=&quot;post&quot;;  
		     oForm.action=contextPath+&quot;struts/suspendRight.jsp&quot;;  
		     
		     var keys=[&quot; , &quot;'&quot; , &quot;codeX&quot; , &quot;'&quot; , &quot;];  
		     var values=[code];
		       
			 if (keys &amp;&amp; values &amp;&amp; (keys.length == values.length)){  
			    for (var i=0; i &lt; keys.length; i++){  
			        var oInput = document.createElement(&quot;input&quot;);  
			        oInput.type=&quot;hidden&quot;;  
			        oInput.name=keys[i];  
			        oInput.value=values[i];  
			        oForm.appendChild(oInput);  
			    }
			 }
			 if(&quot; , &quot;'&quot; , &quot;PREVIEW&quot; , &quot;'&quot; , &quot;==printType){
			     oForm.target=&quot;jasperTarget&quot;;
			     oForm.onSubmit=function(){openSpecfiyWindown(&quot;jasperWindown&quot;)};
		     }
		     document.body.appendChild(oForm);  
		     oForm.submit();   
}




(function($){$(document).ready(function(){$(&quot;#borrowedFlag&quot;).hide();if($(&quot;#isLanguage&quot;).html()==&quot;1&quot;){$(&quot;#borrowedFlag&quot;).show();}});})(jQuery);
 


var patronItemDetails = document.getElementById(&quot;PatronItemDetails&quot;);
var isShowPatronPhotoLoanDesk = document.getElementById(&quot;isShowPatronPhotoLoanDesk&quot;);
var reset = document.getElementById(&quot;reset&quot;);
if(isShowPatronPhotoLoanDesk){
	patronItemDetails.style.marginBottom = &quot;-49px&quot;;
    patronItemDetails.style.marginTop = &quot;-72px&quot;;
    reset.style.marginTop = &quot;-77px&quot;;
}


	.tdlwx{ font-weight:700; font-style:italic; font-family:Verdana, Geneva, sans-serif;white-space:nowrap; width:35px;}


	1

	/inspireapp/


	
	
		
			
				未選擇讀者資料
		
	
	

	

	
	    
	    
 
  
    罰款
  
  
 
 
  
 

	
	
		
	    
 
  
    Penalties
  
  
 
 
  
 

	



 
 
  
    預約可取
  
  
 
 
  
 


 
 
  
    逾期罰金估算
  
  
 
 
  
 




 
  
    讀者基本資訊
  
  
 
 
  
 




 
 







	


  
    
	
	
  
   
  
   
   



 
  
    提示
  
  
 
 
  
 






	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



  
 
  
  
     
  
 
 
  
 


     
 
  
  
     
  
 
 
  
 

   



  




 
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;patronEditForm&quot;);
var radioGroup_RadioGroup = tapestry.byId(&quot;RadioGroup&quot;);

    if ( ! radioGroup_RadioGroup.onChange )
    {
        radioGroup_RadioGroup.onChange = function( value ) {/* do nothing */ };
    }
calendar_lockedDatetimeDatePicker = new Calendar();

	
calendar_lockedDatetimeDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_lockedDatetimeDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).lockedDatetimeDatePicker;
  var value = calendar_lockedDatetimeDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}
calendar_equEndDateDatePicker = new Calendar();

	
calendar_equEndDateDatePicker.initialize([&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u4E00\u6708&quot;, &quot;\u4E8C\u6708&quot;, &quot;\u4E09\u6708&quot;, &quot;\u56DB\u6708&quot;, &quot;\u4E94\u6708&quot;, &quot;\u516D\u6708&quot;, &quot;\u4E03\u6708&quot;, &quot;\u516B\u6708&quot;, &quot;\u4E5D\u6708&quot;, &quot;\u5341\u6708&quot;, &quot;\u5341\u4E00\u6708&quot;, &quot;\u5341\u4E8C\u6708&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  [&quot;\u661F\u671F\u65E5&quot;, &quot;\u661F\u671F\u4E00&quot;, &quot;\u661F\u671F\u4E8C&quot;, &quot;\u661F\u671F\u4E09&quot;, &quot;\u661F\u671F\u56DB&quot;, &quot;\u661F\u671F\u4E94&quot;, &quot;\u661F\u671F\u516D&quot;],
  &quot;yyyy/MM/dd&quot;, 0, false, 1, &quot;Clear&quot;);
calendar_equEndDateDatePicker.onchange = function() {
  var field = tapestry.byId(&quot;patronEditForm&quot;).equEndDateDatePicker;
  var value = calendar_equEndDateDatePicker.formatDate();
	if (field.value != value) {
    field.value = value;
    if (field.onchange) { field.onchange();}
  }
}

tapestry.form.registerForm(&quot;classesForm&quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;PenaltiesDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;SiteEquPenaltiesDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;HoldsListDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;estimatedPenaltiesArea&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;detailsDialog&quot; , &quot;'&quot; , &quot;);
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
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;TextField&quot; , &quot;'&quot; , &quot;);});
// -->




&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear&lt;&lt;一月二月三月四月五月六月七月八月九月十月十一月十二月1920192119221923192419251926192719281929193019311932193319341935193619371938193919401941194219431944194519461947194819491950195119521953195419551956195719581959196019611962196319641965196619671968196919701971197219731974197519761977197819791980198119821983198419851986198719881989199019911992199319941995199619971998199920002001200220032004200520062007200820092010201120122013201420152016201720182019202020212022202320242025202620272028202920302031203220332034203520362037203820392040204120422043204420452046204720482049>>星期日星期一星期二星期三星期四星期五星期六    1234567891011121314151617181920212223242526272829         27 二月, 2024Clear/html[1]&quot;))]</value>
      <webElementGuid>fd095f92-dfe2-4ad2-9da2-8659caf9fd25</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
