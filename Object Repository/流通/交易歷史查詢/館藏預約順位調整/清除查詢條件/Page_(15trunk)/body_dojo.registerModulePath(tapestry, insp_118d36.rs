<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_118d36</name>
   <tag></tag>
   <elementGuidId>1f1dcb05-7537-4853-941b-1a1db983cef2</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#Body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='Body']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>98892a24-34eb-41e7-bc44-cd7140d861b2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>d3047b39-68fa-4d12-81f5-e6848478aac5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>598c51fd-6a95-4fb0-a701-1316023bf001</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>rightmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>87f0e3c7-30d5-481c-bbee-7a57e8f72c8d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bottommargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>caf48348-8347-4de0-91fb-71612ee54170</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bgcolor</name>
      <type>Main</type>
      <value>#ffffff</value>
      <webElementGuid>54610501-7e21-4f7c-ac4c-1fd569600bd5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>cf8f700c-a1c4-451c-8f6b-97e4608b1622</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';

















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = '/inspireapp/ThemePanel/';

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
  	// HTML code to the right of the folder item
  	mainFolderRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'arrowdown.gif&quot;/>',
	// HTML code to the left of the regular item
	mainItemLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the regular item
	mainItemRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the folder item
	folderRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'arrow.gif&quot;/>',
	// HTML code to the left of the regular item
	itemLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the regular item
	itemRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, '&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>'];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, '&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>'];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, '|'];


dojo.require(&quot;tapestry.event&quot;);








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
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = '/inspireapp/images/';
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display ='';
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display ='none';
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-07 14:29:11 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 館藏預約順位調整		
			
	



	jQuery(function () {
		jQuery(&quot;div[id='HiddenDiacritics']&quot;).draggable({
       	 containment: &quot;#box&quot;,
       	 scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id='hiddenDiacritics']&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
  

function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+'&amp;sp=l'+elementId+'&amp;sp='+(checkedElem==true?'T':'F');
	return tapestry.linkOnClick(href,'checkInform', false);
}
	
function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}
    
var refreshTime = 0;   
function apasa() {
  refreshTime = 2500;
  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit('searchForm', 'remakeSearch');
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬 start
	try{
	    var treeJsonData = JSON.parse(j('#locData').text());
	    
	    j('.place').treeoptions({
	        data: treeJsonData,
	        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
	        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	    var treeJsonData1 = JSON.parse(j('#itemClassData').text());
	    
	    j('.place0').treeoptions({
	        data: treeJsonData1,
	        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
	        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	        var treeJsonData2 = JSON.parse(j('#materialTypeData').text());
	        
	        j('.place1').treeoptions({
	            data: treeJsonData2,
	            openImg: '/inspireapp/images/ico_add.gif', // img 路徑
	            cleanImg:'/inspireapp/images/clear.gif' // img 路徑
	        });
        }catch(e){
        }
});





  	
	  
  	
  	
  	

  
  function clickDateTimePicker(obj,comparator){
	  (function($){
		  var h=0,m=0,s=0;
		  if($('#'+comparator).find(&quot;:selected&quot;).val()==&quot;&lt;=&quot;){
			  h=23;
			  m=59;
			  s=59;
		  }
		  if(!$(obj).hasClass('hasDatepicker')){ 
			  $(obj).datetimepicker({
				  	dateFormat: 'yy/mm/dd',
					timeFormat: 'HH:mm:ss',
					stepHour: 1,
					stepMinute: 1,
					stepSecond: 1,
					hour:h,
					minute:m,
					second:s,
					changeMonth: true,
	                changeYear: true,
	                timeInput: true
				});
			}
		  $(obj).focus();	
	  })(jQuery);
  }
  

  
  
       
          refreshTime = 0;
       
  
  



 查詢 
		  瀏覽 
      















































































	



	
	
	
	
		
			查詢條件
			
							 
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
		      					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

          					
							
							
				      			
				      			
				      			  
				      			  
				      			  	
				      			  
				      			
				      			
	       					
				      			  
				      			
				      			
	       					
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							 
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
				      			
	       	 				
				      			  
				      			
				      			
	       	 				
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							
								
					      			
					      			
					      			  
					      			  
					      				
					      			  
					      			
	      					
	       		 		
					      			  
					      			
	      					
	       		 		
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
	      					
	       	 				
				      			  
				      			
	      					
	       	 				
							
						
						 
		                    
		                    	
		                    		限制條件
		                    	
		                    	
		                    		
		                    			                    
		                    
		                    
		
		
					
					
						 
						
						
						
						
						
						
						
					
		
	

   {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
   
		            
		              {&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}
		                
		                  限制條件
		                  
		                
						
						
						
						
						館藏狀態:
						

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

						
						
						
						
						
		                
						
				        館藏地:
						
							

						
						
						
							資料類型:
							
					   		
						
						
						
				        取書館別:
						
							

						
						
						
						
						
						
						
						
						
						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
正題名
起始日期
結束日期
完成日期
讀者姓名

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
						       
條碼號
	
						      
						      
						   
					       
升冪
降冪

                           
                          
		                    
		                    



		                    			       
		                    
		                    
		                    
                                
						    		
						    	
		                    
					      
					      
					      
							
							  
						        
						          
						            
						              
						                
						                  符號表:
						                  
						                
						                
						                  
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


						                
						              
						          
						          
						        
						      
						  						      	
					      
					     
					    
					 
					 
					 
					 
					   
         
			 			
             
			 
			 
		
  
  
  
   
			
					
			
		
		 
10
20
50
100
500

		 
			
	             
	             
	               3
	  			   筆
	              
				 (0.402s) •
	
			 
	        
	
				
  
  	var exMsg='頁碼錯誤，請重新輸入'
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == 'ro')
          Tapestry.submit_form('CautareRapida', 'langFirst');
      if (document.CautareRapida.languages.value == 'ru')
          Tapestry.submit_form('CautareRapida', 'langSecond');
      if (document.CautareRapida.languages.value == 'en')
          Tapestry.submit_form('CautareRapida', 'langThird');
      if (document.CautareRapida.languages.value == 'ch')
          Tapestry.submit_form('CautareRapida', 'langFourth');
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = '1';
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		
	 
  

    
    
    
    

    	
	
		
	
	
		
		
			序號
			
			系統識別號
			條碼號
			館藏地
			資料類型
			書目資訊
			讀者人數
			
		
		
		
	
		
			
				1
			
			
				
				
			
			
			
			
				168008			
			
				
					0000000034
			
				台中總館期刊區			
			
				裝訂期刊	
			
				Acta oto-laryngologica  123 ; 321 ; 222 ; 222 Oslo [etc.]Scandinavian University Press [etc.]c1918- 
			
				2
			
			
			
		
			
				2
			
			
				
				
			
			
			
			
				29599			
			
				
					20210331
			
				台中總館書庫			
			
				圖書	
			
				好一個年輕的下午 / 林貴真著臺北市 : 爾雅, 民82[1993]
			
				1
			
			
			
		
			
				3
			
			
				
				
			
			
			
			
				198082			
			
				
					012010022986
			
				台中總館書庫			
			
				圖書	
			
				Java concepts /Cay Horstmann.[New York?] :Wiley,c2005.
			
				2
			
			
			
		
	

	
	

	

	

		
		
						
			          		  
					   
  
  	var exMsg='頁碼錯誤，請重新輸入'
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == 'ro')
          Tapestry.submit_form('CautareRapida', 'langFirst');
      if (document.CautareRapida.languages.value == 'ru')
          Tapestry.submit_form('CautareRapida', 'langSecond');
      if (document.CautareRapida.languages.value == 'en')
          Tapestry.submit_form('CautareRapida', 'langThird');
      if (document.CautareRapida.languages.value == 'ch')
          Tapestry.submit_form('CautareRapida', 'langFourth');
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = '1';
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
						
		
			 
			報表 
				
				
				
			
		
	
		 

  

 
 
 

         

 
  
    Go To Page
  
  
 
 
  
 



          
		  
		    
		  

				

// 	dojo.event.topic.subscribe('info', closeDialogComponent('AssignedReports'));
	dojo.event.topic.subscribe('info',function(msg){closeDialogComponent('AssignedReports');});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink('refresher');
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw ('myMenuID', myMenu, 'hbr', cmThemePanel, 'ThemePanel');
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($('#activeTime').text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$('.marquee').css('display','block');
		}else{
			$('.marquee').css('display','none');
		}
		
		$('.marquee').marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log('關閉跑馬燈 &amp; 重新檢查');
		$('.marquee').css('display','none');
	}
	
})(jQuery);
 

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
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
                tapestry.formEvent127597497=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
                tapestry.formEvent203034624=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
                tapestry.formEvent1300514466=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
                tapestry.formEvent494339521=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
                tapestry.formEvent454372361=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
                tapestry.formEvent180190125=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
                tapestry.formEvent1578100515=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
                tapestry.formEvent509154799=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
                tapestry.formEvent173206720=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
closeDialogComponent('TinreadDialog');
try {
  attachFocus('field1');
 }
 catch(e) {}
closeDialogComponent('AssignedReports');
closeDialogComponent('ReportStatus');
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)</value>
      <webElementGuid>9e514669-1813-48bd-bdb8-4ff46b949efc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>dd9898c2-9659-4c05-acca-45c56527827a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>e0afa217-9727-430f-9bde-d8debc23e143</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>ca5f9fea-d6e2-4915-8d4c-3336ced76bb6</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[@id = 'Body' and (text() = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;

















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = &quot; , &quot;'&quot; , &quot;/inspireapp/ThemePanel/&quot; , &quot;'&quot; , &quot;;

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
  	// HTML code to the right of the folder item
  	mainFolderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrowdown.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	mainItemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	mainItemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the folder item
	folderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrow.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	itemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	itemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot;];


dojo.require(&quot;tapestry.event&quot;);








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
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;;
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-07 14:29:11 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 館藏預約順位調整		
			
	



	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;HiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
       	 containment: &quot;#box&quot;,
       	 scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
  

function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;&amp;sp=&quot; , &quot;'&quot; , &quot;+(checkedElem==true?&quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;F&quot; , &quot;'&quot; , &quot;);
	return tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;checkInform&quot; , &quot;'&quot; , &quot;, false);
}
	
function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}
    
var refreshTime = 0;   
function apasa() {
  refreshTime = 2500;
  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakeSearch&quot; , &quot;'&quot; , &quot;);
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬 start
	try{
	    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	    var treeJsonData1 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#itemClassData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place0&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData1,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	        var treeJsonData2 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#materialTypeData&quot; , &quot;'&quot; , &quot;).text());
	        
	        j(&quot; , &quot;'&quot; , &quot;.place1&quot; , &quot;'&quot; , &quot;).treeoptions({
	            data: treeJsonData2,
	            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	        });
        }catch(e){
        }
});





  	
	  
  	
  	
  	

  
  function clickDateTimePicker(obj,comparator){
	  (function($){
		  var h=0,m=0,s=0;
		  if($(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+comparator).find(&quot;:selected&quot;).val()==&quot;&lt;=&quot;){
			  h=23;
			  m=59;
			  s=59;
		  }
		  if(!$(obj).hasClass(&quot; , &quot;'&quot; , &quot;hasDatepicker&quot; , &quot;'&quot; , &quot;)){ 
			  $(obj).datetimepicker({
				  	dateFormat: &quot; , &quot;'&quot; , &quot;yy/mm/dd&quot; , &quot;'&quot; , &quot;,
					timeFormat: &quot; , &quot;'&quot; , &quot;HH:mm:ss&quot; , &quot;'&quot; , &quot;,
					stepHour: 1,
					stepMinute: 1,
					stepSecond: 1,
					hour:h,
					minute:m,
					second:s,
					changeMonth: true,
	                changeYear: true,
	                timeInput: true
				});
			}
		  $(obj).focus();	
	  })(jQuery);
  }
  

  
  
       
          refreshTime = 0;
       
  
  



 查詢 
		  瀏覽 
      















































































	



	
	
	
	
		
			查詢條件
			
							 
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
		      					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

          					
							
							
				      			
				      			
				      			  
				      			  
				      			  	
				      			  
				      			
				      			
	       					
				      			  
				      			
				      			
	       					
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							 
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
				      			
	       	 				
				      			  
				      			
				      			
	       	 				
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							
								
					      			
					      			
					      			  
					      			  
					      				
					      			  
					      			
	      					
	       		 		
					      			  
					      			
	      					
	       		 		
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
	      					
	       	 				
				      			  
				      			
	      					
	       	 				
							
						
						 
		                    
		                    	
		                    		限制條件
		                    	
		                    	
		                    		
		                    			                    
		                    
		                    
		
		
					
					
						 
						
						
						
						
						
						
						
					
		
	

   {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
   
		            
		              {&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}
		                
		                  限制條件
		                  
		                
						
						
						
						
						館藏狀態:
						

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

						
						
						
						
						
		                
						
				        館藏地:
						
							

						
						
						
							資料類型:
							
					   		
						
						
						
				        取書館別:
						
							

						
						
						
						
						
						
						
						
						
						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
正題名
起始日期
結束日期
完成日期
讀者姓名

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
						       
條碼號
	
						      
						      
						   
					       
升冪
降冪

                           
                          
		                    
		                    



		                    			       
		                    
		                    
		                    
                                
						    		
						    	
		                    
					      
					      
					      
							
							  
						        
						          
						            
						              
						                
						                  符號表:
						                  
						                
						                
						                  
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


						                
						              
						          
						          
						        
						      
						  						      	
					      
					     
					    
					 
					 
					 
					 
					   
         
			 			
             
			 
			 
		
  
  
  
   
			
					
			
		
		 
10
20
50
100
500

		 
			
	             
	             
	               3
	  			   筆
	              
				 (0.402s) •
	
			 
	        
	
				
  
  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		
	 
  

    
    
    
    

    	
	
		
	
	
		
		
			序號
			
			系統識別號
			條碼號
			館藏地
			資料類型
			書目資訊
			讀者人數
			
		
		
		
	
		
			
				1
			
			
				
				
			
			
			
			
				168008			
			
				
					0000000034
			
				台中總館期刊區			
			
				裝訂期刊	
			
				Acta oto-laryngologica  123 ; 321 ; 222 ; 222 Oslo [etc.]Scandinavian University Press [etc.]c1918- 
			
				2
			
			
			
		
			
				2
			
			
				
				
			
			
			
			
				29599			
			
				
					20210331
			
				台中總館書庫			
			
				圖書	
			
				好一個年輕的下午 / 林貴真著臺北市 : 爾雅, 民82[1993]
			
				1
			
			
			
		
			
				3
			
			
				
				
			
			
			
			
				198082			
			
				
					012010022986
			
				台中總館書庫			
			
				圖書	
			
				Java concepts /Cay Horstmann.[New York?] :Wiley,c2005.
			
				2
			
			
			
		
	

	
	

	

	

		
		
						
			          		  
					   
  
  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
						
		
			 
			報表 
				
				
				
			
		
	
		 

  

 
 
 

         

 
  
    Go To Page
  
  
 
 
  
 



          
		  
		    
		  

				

// 	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;));
	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,function(msg){closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw (&quot; , &quot;'&quot; , &quot;myMenuID&quot; , &quot;'&quot; , &quot;, myMenu, &quot; , &quot;'&quot; , &quot;hbr&quot; , &quot;'&quot; , &quot;, cmThemePanel, &quot; , &quot;'&quot; , &quot;ThemePanel&quot; , &quot;'&quot; , &quot;);
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($(&quot; , &quot;'&quot; , &quot;#activeTime&quot; , &quot;'&quot; , &quot;).text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		}else{
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		}
		
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log(&quot; , &quot;'&quot; , &quot;關閉跑馬燈 &amp; 重新檢查&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
	}
	
})(jQuery);
 

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
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
                tapestry.formEvent127597497=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
                tapestry.formEvent203034624=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
                tapestry.formEvent1300514466=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
                tapestry.formEvent494339521=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
                tapestry.formEvent454372361=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
                tapestry.formEvent180190125=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
                tapestry.formEvent1578100515=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
                tapestry.formEvent509154799=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
                tapestry.formEvent173206720=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);
try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;ReportStatus&quot; , &quot;'&quot; , &quot;);
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)&quot;) or . = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;

















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = &quot; , &quot;'&quot; , &quot;/inspireapp/ThemePanel/&quot; , &quot;'&quot; , &quot;;

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
  	// HTML code to the right of the folder item
  	mainFolderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrowdown.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	mainItemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	mainItemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the folder item
	folderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrow.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	itemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	itemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot;];


dojo.require(&quot;tapestry.event&quot;);








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
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;;
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-07 14:29:11 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 館藏預約順位調整		
			
	



	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;HiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
       	 containment: &quot;#box&quot;,
       	 scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
  

function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;&amp;sp=&quot; , &quot;'&quot; , &quot;+(checkedElem==true?&quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;F&quot; , &quot;'&quot; , &quot;);
	return tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;checkInform&quot; , &quot;'&quot; , &quot;, false);
}
	
function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}
    
var refreshTime = 0;   
function apasa() {
  refreshTime = 2500;
  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakeSearch&quot; , &quot;'&quot; , &quot;);
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬 start
	try{
	    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	    var treeJsonData1 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#itemClassData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place0&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData1,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
	    }catch(e){
	    }
	    try{
	        var treeJsonData2 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#materialTypeData&quot; , &quot;'&quot; , &quot;).text());
	        
	        j(&quot; , &quot;'&quot; , &quot;.place1&quot; , &quot;'&quot; , &quot;).treeoptions({
	            data: treeJsonData2,
	            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	        });
        }catch(e){
        }
});





  	
	  
  	
  	
  	

  
  function clickDateTimePicker(obj,comparator){
	  (function($){
		  var h=0,m=0,s=0;
		  if($(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+comparator).find(&quot;:selected&quot;).val()==&quot;&lt;=&quot;){
			  h=23;
			  m=59;
			  s=59;
		  }
		  if(!$(obj).hasClass(&quot; , &quot;'&quot; , &quot;hasDatepicker&quot; , &quot;'&quot; , &quot;)){ 
			  $(obj).datetimepicker({
				  	dateFormat: &quot; , &quot;'&quot; , &quot;yy/mm/dd&quot; , &quot;'&quot; , &quot;,
					timeFormat: &quot; , &quot;'&quot; , &quot;HH:mm:ss&quot; , &quot;'&quot; , &quot;,
					stepHour: 1,
					stepMinute: 1,
					stepSecond: 1,
					hour:h,
					minute:m,
					second:s,
					changeMonth: true,
	                changeYear: true,
	                timeInput: true
				});
			}
		  $(obj).focus();	
	  })(jQuery);
  }
  

  
  
       
          refreshTime = 0;
       
  
  



 查詢 
		  瀏覽 
      















































































	



	
	
	
	
		
			查詢條件
			
							 
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
		      					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

          					
							
							
				      			
				      			
				      			  
				      			  
				      			  	
				      			  
				      			
				      			
	       					
				      			  
				      			
				      			
	       					
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							 
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
				      			
	       	 				
				      			  
				      			
				      			
	       	 				
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							
								
					      			
					      			
					      			  
					      			  
					      				
					      			  
					      			
	      					
	       		 		
					      			  
					      			
	      					
	       		 		
						
						
							
and
or
not

							
條碼號
正題名
系統識別號
起始日期
結束日期
完成日期
讀者姓名
讀者證號

							
							 	
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

	      					
							 
							 
				      			
				      			
				      			  
				      			  
				      				
				      			  
				      			
	      					
	       	 				
				      			  
				      			
	      					
	       	 				
							
						
						 
		                    
		                    	
		                    		限制條件
		                    	
		                    	
		                    		
		                    			                    
		                    
		                    
		
		
					
					
						 
						
						
						
						
						
						
						
					
		
	

   {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
   
		            
		              {&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}
		                
		                  限制條件
		                  
		                
						
						
						
						
						館藏狀態:
						

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

						
						
						
						
						
		                
						
				        館藏地:
						
							

						
						
						
							資料類型:
							
					   		
						
						
						
				        取書館別:
						
							

						
						
						
						
						
						
						
						
						
						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
正題名
起始日期
結束日期
完成日期
讀者姓名

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
						       
條碼號
	
						      
						      
						   
					       
升冪
降冪

                           
                          
		                    
		                    



		                    			       
		                    
		                    
		                    
                                
						    		
						    	
		                    
					      
					      
					      
							
							  
						        
						          
						            
						              
						                
						                  符號表:
						                  
						                
						                
						                  
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


						                
						              
						          
						          
						        
						      
						  						      	
					      
					     
					    
					 
					 
					 
					 
					   
         
			 			
             
			 
			 
		
  
  
  
   
			
					
			
		
		 
10
20
50
100
500

		 
			
	             
	             
	               3
	  			   筆
	              
				 (0.402s) •
	
			 
	        
	
				
  
  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						      
			
		
	 
  

    
    
    
    

    	
	
		
	
	
		
		
			序號
			
			系統識別號
			條碼號
			館藏地
			資料類型
			書目資訊
			讀者人數
			
		
		
		
	
		
			
				1
			
			
				
				
			
			
			
			
				168008			
			
				
					0000000034
			
				台中總館期刊區			
			
				裝訂期刊	
			
				Acta oto-laryngologica  123 ; 321 ; 222 ; 222 Oslo [etc.]Scandinavian University Press [etc.]c1918- 
			
				2
			
			
			
		
			
				2
			
			
				
				
			
			
			
			
				29599			
			
				
					20210331
			
				台中總館書庫			
			
				圖書	
			
				好一個年輕的下午 / 林貴真著臺北市 : 爾雅, 民82[1993]
			
				1
			
			
			
		
			
				3
			
			
				
				
			
			
			
			
				198082			
			
				
					012010022986
			
				台中總館書庫			
			
				圖書	
			
				Java concepts /Cay Horstmann.[New York?] :Wiley,c2005.
			
				2
			
			
			
		
	

	
	

	

	

		
		
						
			          		  
					   
  
  	var exMsg=&quot; , &quot;'&quot; , &quot;頁碼錯誤，請重新輸入&quot; , &quot;'&quot; , &quot;
  
   

    function onSelectChange() {         
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFirst&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langSecond&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langThird&quot; , &quot;'&quot; , &quot;);
      if (document.CautareRapida.languages.value == &quot; , &quot;'&quot; , &quot;ch&quot; , &quot;'&quot; , &quot;)
          Tapestry.submit_form(&quot; , &quot;'&quot; , &quot;CautareRapida&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;langFourth&quot; , &quot;'&quot; , &quot;);
    }
j(document).ready(function(){
	
    	j(&quot;#cp&quot;).keydown(function (e){
    	    if(e.keyCode == 13){
    	    	gotofunction();
    	    	return false;
    	    }
    	});
    	
    	j(&quot;.pages_btn&quot;).click(function(){
    		gotofunction();
    	});
    	
   		j(&quot;#cp&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp_0&quot;).val(j(&quot;#cp&quot;).val());
   		});
   		
   		j(&quot;#cp_0&quot;).keyup(function(){
   		//alert(j(&quot;#cp&quot;).val());
   			j(&quot;#cp&quot;).val(j(&quot;#cp_0&quot;).val());
   		});
    	
    });
    
    function gotofunction() {
    	var h=j(&quot;#go2&quot;).attr(&quot;href&quot;);
  		var head=h.substring(0,h.indexOf(&quot;?&quot;));
  		var foot=h.substring(h.indexOf(&quot;&amp;&quot;)+1,h.length);
  		var body=&quot;?sp=&quot;+j(&quot;#cp&quot;).val()+&quot;&amp;&quot;;
  		var hr=head+body+foot;
  		if(isNaN(j(&quot;#cp&quot;).val())){
  			alert(exMsg);
  			document.getElementById(&quot;cp&quot;).value = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
  		}else{
  			j(&quot;#go2&quot;).attr(&quot;href&quot;,hr)
      		j(&quot;#go2&quot;).click();
  		}
    }
  
						
							
							
								 
								    			
								  
							
							
														
								
								
									
									  
									     1  		       
									   
									               
									  
								           
								
							
							
							 
									                 
									 
								
							
								
								
								跳至頁碼:								
								 
								
								
							 GO
													
						
						
							 
			          
			
					  
			
					
			
			
				
						
		
			 
			報表 
				
				
				
			
		
	
		 

  

 
 
 

         

 
  
    Go To Page
  
  
 
 
  
 



          
		  
		    
		  

				

// 	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;));
	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,function(msg){closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw (&quot; , &quot;'&quot; , &quot;myMenuID&quot; , &quot;'&quot; , &quot;, myMenu, &quot; , &quot;'&quot; , &quot;hbr&quot; , &quot;'&quot; , &quot;, cmThemePanel, &quot; , &quot;'&quot; , &quot;ThemePanel&quot; , &quot;'&quot; , &quot;);
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($(&quot; , &quot;'&quot; , &quot;#activeTime&quot; , &quot;'&quot; , &quot;).text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		}else{
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		}
		
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log(&quot; , &quot;'&quot; , &quot;關閉跑馬燈 &amp; 重新檢查&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
	}
	
})(jQuery);
 

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
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/ReservationsItemSearch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
                tapestry.formEvent127597497=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent127597497&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
                tapestry.formEvent203034624=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent203034624&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
                tapestry.formEvent1300514466=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1300514466&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
                tapestry.formEvent494339521=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent494339521&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
                tapestry.formEvent454372361=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent454372361&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
                tapestry.formEvent180190125=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent180190125&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
                tapestry.formEvent1578100515=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1578100515&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
                tapestry.formEvent509154799=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent509154799&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
                tapestry.formEvent173206720=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/ReservationsItemSearch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent173206720&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);
try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;ReportStatus&quot; , &quot;'&quot; , &quot;);
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)&quot;))]</value>
      <webElementGuid>1d1cddc0-0150-413f-9db2-32aeff2c30d0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
