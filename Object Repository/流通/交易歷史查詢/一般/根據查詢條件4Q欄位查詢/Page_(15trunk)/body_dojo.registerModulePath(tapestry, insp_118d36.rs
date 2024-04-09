<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_118d36</name>
   <tag></tag>
   <elementGuidId>eea9abc0-572a-4ec6-bb09-5031ad3ae08c</elementGuidId>
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
      <webElementGuid>f579ed24-740f-466f-8532-962920985df2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>c51d4518-aa05-413f-995a-69c933bcb13b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>27ff2266-d11a-4dab-bf61-38ab0ae6841e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>rightmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>cd6b5ba1-9e87-48c2-8c81-0075a0d0fa5f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bottommargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>197c4028-cb45-4913-b05e-661f1e159f16</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bgcolor</name>
      <type>Main</type>
      <value>#ffffff</value>
      <webElementGuid>8d677820-5f23-4844-900d-299e2786af22</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>5dfe4258-7b92-4128-984d-d47a14152537</webElementGuid>
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
				
				最近登入:2024-03-06 16:15:04 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 一般搜尋交易		
			
	
		



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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
		      					
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							 
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
		                  
		                
						
						
						
						
						交易狀態類型:
						

進行中
已結束
續借/延長

						
						交易紀錄類型:
						

借閱
隔夜借閱
預約保留
預約
內部流通

						
						交易結束模式:
						

一般
預約逾期未取
讀者取消
館員取消
下一筆借出
遺失
聲明歸還
還書箱
自助借還書機
預約重新連結至題名
結束盤點
預約駁回
預約核准
書移送到非點收館
預約停權

						
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
							
					   		
						
						
						
						
						
						
						 單位所系:
						  
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
單位所系_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree('window.ddepartment', 'messages', '/inspireapp/images/'); 
window.ddepartment.add(0,-1,'單位所系'); 
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
  
  
  


						
						
						
						 讀者身份類別:
						 

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

						
						
						
							館藏使用類型:
							

eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試

						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
讀者證號
讀者姓名
館藏使用類型

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
							  
讀者姓名
條碼號
起始日期
結束日期
完成日期
館藏使用類型

						       
						      
						   
					       
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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
                tapestry.formEvent337563576=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
                tapestry.formEvent1057424483=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
                tapestry.formEvent876452286=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
                tapestry.formEvent118432723=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
                tapestry.formEvent1615450550=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
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
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
                tapestry.formEvent2013338881=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
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
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
                tapestry.formEvent1256464714=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
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
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
                tapestry.formEvent83350220=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
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
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
                tapestry.formEvent950363360=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
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
      <webElementGuid>4429a127-e60e-409d-a8bb-91eef93b9dcd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>abcc3393-4ace-4d0c-a0e9-ce12a2fcdc4f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>f9ae7446-7ea1-4c2f-aabe-777a61a25149</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>fcb51151-643f-4bce-a594-2df71a017cc8</webElementGuid>
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
				
				最近登入:2024-03-06 16:15:04 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 一般搜尋交易		
			
	
		



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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
		      					
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							 
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
		                  
		                
						
						
						
						
						交易狀態類型:
						

進行中
已結束
續借/延長

						
						交易紀錄類型:
						

借閱
隔夜借閱
預約保留
預約
內部流通

						
						交易結束模式:
						

一般
預約逾期未取
讀者取消
館員取消
下一筆借出
遺失
聲明歸還
還書箱
自助借還書機
預約重新連結至題名
結束盤點
預約駁回
預約核准
書移送到非點收館
預約停權

						
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
							
					   		
						
						
						
						
						
						
						 單位所系:
						  
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
單位所系_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree(&quot; , &quot;'&quot; , &quot;window.ddepartment&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(0,-1,&quot; , &quot;'&quot; , &quot;單位所系&quot; , &quot;'&quot; , &quot;); 
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
  
  
  


						
						
						
						 讀者身份類別:
						 

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

						
						
						
							館藏使用類型:
							

eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試

						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
讀者證號
讀者姓名
館藏使用類型

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
							  
讀者姓名
條碼號
起始日期
結束日期
完成日期
館藏使用類型

						       
						      
						   
					       
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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
                tapestry.formEvent337563576=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
                tapestry.formEvent1057424483=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
                tapestry.formEvent876452286=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
                tapestry.formEvent118432723=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
                tapestry.formEvent1615450550=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
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
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
                tapestry.formEvent2013338881=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
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
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
                tapestry.formEvent1256464714=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
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
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
                tapestry.formEvent83350220=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
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
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
                tapestry.formEvent950363360=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
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
				
				最近登入:2024-03-06 16:15:04 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 交易歷史查詢 > 一般搜尋交易		
			
	
		



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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
		      					
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							 
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
讀者帳號
讀者證號
讀者姓名
正題名
起始日期
結束日期
完成日期
預約可取夾書單號
採購註記
執行交易館員帳號
結束交易館員帳號
索書號
作者

							
							 	
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
		                  
		                
						
						
						
						
						交易狀態類型:
						

進行中
已結束
續借/延長

						
						交易紀錄類型:
						

借閱
隔夜借閱
預約保留
預約
內部流通

						
						交易結束模式:
						

一般
預約逾期未取
讀者取消
館員取消
下一筆借出
遺失
聲明歸還
還書箱
自助借還書機
預約重新連結至題名
結束盤點
預約駁回
預約核准
書移送到非點收館
預約停權

						
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
							
					   		
						
						
						
						
						
						
						 單位所系:
						  
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
單位所系_New Item_New Item1234神資圖書館行政單位其他其他其他學校行政台中總館北港圖書分館安南圖書分館水湳圖書分館學校行政單位北港分部行政單位學校研究中心學術單位其他醫學院其他醫學系其他醫學系一年A班醫學檢驗生物技術學系生物醫學影像暨放射科學學系生物醫學研究所碩士班醫學檢驗生物技術學系碩士班生物醫學影像暨放射科學學系碩士班國際生物醫學碩士學位學程臨床醫學研究所基礎醫學研究所免疫學研究所癌症生物學研究所神經科學與認知科學研究所生物醫學研究所博士班癌症生物與藥物研發博士學位學程老化醫學博士學位學程轉譯醫學博士學位學程生醫科技產業博士學位學程中醫學院New Item中醫學系中醫學系甲組中醫學系乙組中國藥學暨中藥資源學系學士後中醫學系中醫學系碩士班中西醫結合研究所碩士班針灸研究所碩士班中國藥學暨中藥資源學系碩士班國際針灸碩士學位學程中獸醫碩士學位學程中醫學系博士班中西醫結合研究所博士班針灸研究所博士班中國藥學暨中藥資源學系博士班藥學院藥學系藥學系碩士班藥學系博士班生技製藥產業博士學位學程公共衛生學院公共衛生學系職業安全與衛生學系醫務管理學系公共衛生學院大一不分系健康風險管理學系公共衛生學系碩士班公共衛生國際碩士學位學程職業安全與衛生學系碩士班職業安全與衛生學系碩士在職專班醫務管理學系碩士班醫務管理學系碩士在職專班健康風險管理學系碩士班生物統計研究所碩士班公共衛生學系博士班單位系所匯入醫學工程與復健科技產業博士學位學程生物醫學工程碩士學位學程健康照護學院物理治療學系護理學系運動醫學系口腔衛生學系二年制護理學系在職專班二年制呼吸治療學系在職專班物理治療學系復健科學碩士班護理學系碩士班護理學系跨領域長期照護碩士在職專班健康科技產業博士學位學程生技製藥暨食品科學院營養學系生物科技學系藥用化妝品學系營養學系碩士班生物科技學系碩士班藥用化妝品學系碩士班製藥碩士學位學程食品暨藥物安全碩士學位學程營養學系博士班生物科技學系博士班新藥開發研究所博士班生物科技產業博士學位學程人文與科技學院科技法律碩士學位學程其他科技管理碩士學位學程牙醫學院牙醫學系牙醫學系碩士班牙醫學系口腔醫學產業碩士班牙醫學系博士班通識教育中心通識教育中心附設機構中國附醫附醫研究中心內科部外科部神經外科部骨科部泌尿部婦產部神經部耳鼻喉部皮膚科牙醫部精神醫學部復健部麻醉部臨床營養科中醫部中國附醫行政單位社會工作室眼科部兒童醫院病理部基因醫學部預防醫學中心醫學研究部教學部急症暨外傷中心護理部藥劑部醫學影像部檢驗醫學部核子醫學科神經精神醫學中心醫療品質部癌症中心附醫-北港附醫北港附設醫院附醫-豐原分院豐原分院附醫-豐原醫務室豐原醫務室附醫-台中東區分院台中東區分院附醫-台北分院台北分院附醫-中監培德醫院中監培德醫院附醫-中科園區員工診所中科園區員工診所附醫-草屯分院草屯分院附醫-陽光精神科醫院陽光精神科醫院附醫-地利村門診中心地利村門診中心附醫-安南醫院安南醫院校外單位館際合作NDDS館際合作互借協議聯盟中盟-大葉大學中盟-中山醫大中盟-中臺科大中盟-中興大學中盟-台中教大中盟-弘光科大中盟-亞洲大學中盟-東海大學中盟-建國科大中盟-暨南大學中盟-逢甲大學中盟-朝陽科大中盟-勤益科大中盟-彰化師大中盟-靜宜大學中盟-嶺東科大中盟-台中科大中盟-聯合大學中盟-明道大學中盟-南開科大中盟-修平科大中盟-育達科大中盟-僑光科大校外校外人士test123test234test777
  
  
window.ddepartment = new dTree(&quot; , &quot;'&quot; , &quot;window.ddepartment&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.ddepartment.add(0,-1,&quot; , &quot;'&quot; , &quot;單位所系&quot; , &quot;'&quot; , &quot;); 
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
  
  
  


						
						
						
						 讀者身份類別:
						 

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

						
						
						
							館藏使用類型:
							

eeewww
testerer
一般(Normal)
展示(Exhibition)
指參(Academic Reserve)
新書(New Book)
測試

						
						
						

						
						
						
						    
		                
		              
					  
		          
			 
	
  
  
  
  
  
  
  
  
  
  
  
  
  
    
      瀏覽條件:
      
      	
條碼號
讀者證號
讀者姓名
館藏使用類型

   	
	起始以:
	
	    
	    			
	      			
	      				 
						

//&lt;![CDATA[

							document.getElementById(&quot;listField&quot;).focus();
						
//]]&gt;


	      			
	    
	    
		 
      
    
    
       
    
  
  
  
  
  
  
  
  
  
      
  					
					 
					     
					      
					       
					 		
						                  排序條件:  
							  
							  
讀者姓名
條碼號
起始日期
結束日期
完成日期
館藏使用類型

						       
						      
						   
					       
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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=Sitem_number&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/pages/SearchTransactions,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
                tapestry.formEvent337563576=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent337563576&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
                tapestry.formEvent1057424483=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent1057424483&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
                tapestry.formEvent876452286=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent876452286&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
                tapestry.formEvent118432723=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent118432723&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
                tapestry.formEvent1615450550=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
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
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent1615450550&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
                tapestry.formEvent2013338881=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
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
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent2013338881&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
                tapestry.formEvent1256464714=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
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
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent1256464714&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
                tapestry.formEvent83350220=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
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
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent83350220&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
                tapestry.formEvent950363360=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;circulation/pages/SearchTransactions/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent950363360&quot;);
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
      <webElementGuid>7a6ab8ba-2659-4c2c-acfd-1cb90d01c5d4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
