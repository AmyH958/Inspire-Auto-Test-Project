import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2'))

WebUI.scrollToElement(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3'), 0)

WebUI.enableSmartWait()

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3'), '新增')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td_'), '選擇控制項:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_Prefix'), '[Prefix]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_Sufix'), '[Sufix]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_Number'), '[Number]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_Location'), '[Location]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_Patron class'), '[Patron class]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/b_National ID number'), '[National ID number]')

WebUI.verifyElementVisible(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td_()'), '館藏排序規則:\n \n (拖曳部份)')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/img__Any_5'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_Prefix'), '[Prefix]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_Sufix'), '[Sufix]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_Number'), '[Number]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_Location'), '[Location]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_Patron class'), '[Patron class]')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/font_National ID number'), '[National ID number]')

WebUI.setText(findTestObject('Page_(15trunk)/input__TextField_0'), '5')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td__1'))

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input__TextField_1'), '10')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_010203040506070809101111112131415161_828bdf'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_010203040506070809101111112131415161_828bdf'), 
    '2', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_010203040506070809101111112131415161_828bdf'), 
    '01大學生一年級\n02大學生二年級\n03大學生三年級\n04大學生四年級\n05大學生五年級\n06大學生六年級\n07大學生七年級\n08大學生八年級\n09大學畢業生\n10延畢生\n111\n11碩士班一年級\n12碩士班二年級\n13碩士班三年級\n14碩士班四年級\n15博士班一年級\n16博士班二年級\n17博士班三年級\n18博士班四年級\n19博士班五年級\n20博士班六年級\n21博士班七年級\n22預研生\n23碩博休學生\n24專任教師\n25兼任教師\n26學校職員\n27學校研究助理\n28交換師生\n29研究計劃\n30訪問學人\n31館內使用\n32主治醫師\n33住院醫師\n34代訓醫師\n35兼任醫師\n36實習醫學生\n37行政人員\n38研究員\n39醫檢師\n40藥師\n41護理師\n42技術師\n43治療師\n44社工師\n45營養師\n46其他人員\n47醫院研究助理\n48附醫實習學生(不借書)\n49附醫實習學生(可借書)\n50推廣教育學員\n51館際合作\n52中區聯盟\n53彰雲嘉聯盟\n54附醫護生\n55校友\n56退休人員\n57尊爵會員\n58個人會員\n59員眷\n60電子資源權限(校外)\n61無權限畢業生\n62當年度畢業生\nererrrr\nERM同步\nERM測試身分\nNick\nqkwmvud\n日間部四年制學生')

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input__TextField_2'), '1234567890')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td__1_2'))

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/input__TextField_2'), '1234567890')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3_4'), '修改/存檔')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/div_'), '更動已儲存')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3_4_5'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/span_'))

WebUI.takeFullPageScreenshotAsCheckpoint('新增讀者證卡成功')

WebUI.scrollToElement(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td__1_2'), 0)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td__1_2'), '碼:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td_1234567890'), '1234567890')

WebUI.takeFullPageScreenshotAsCheckpoint('確認新增的證卡')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/td__1_2_3'), '台中總館書庫')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_()T-Pnew item 10L4020230418new item _5ed051_1'), 
    '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/select_()T-Pnew item 10L4020230418new item _5ed051_1'), 
    '台中總館書庫\n台中總館閱覽組\n台中總館流通櫃檯\n台中總館博碩士論文區\n台中總館視聽區(獨立專區)\n台中總館研究計劃專書\nT-P\nnew item 10\nL40\n20230418\nnew item 16\nnew item 3\nnew item 1\nLB-S\n台中總館新書展示區\n台中總館參考室\n台中總館本校教師升等資料區\n台中總館國考書區(限館內閱覽)\n台中總館線上資料\n台中總館教師指定參考書(限館內閱覽)\n台中總館期刊區\n台中總館視聽區\n台中總館\nnew item 13\n安南醫院\n龍華科技大學圖書館\n豐原分院圖書區\n豐原分院\n語文教學中心圖書室\n語文教學中心\n台中總館本校教職優良教材區\n台中總館密閉書庫\n台中總館本校教職論著\n台中總館人文專書區-醫療品質(限館內閱覽)\nnew item 7\n台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)\n台中總館特藏室\n台中總館探索史懷哲之路專書區\n台中總館人文專書區-醫療史(限館內閱覽)\n台中總館期刊複本櫃\n北港分館視聽區(限館內閱覽)\n北港分館書庫\n北港分館指參(限館內閱覽)\n北港分館\n取書櫃2\nB007\ncircd\nNPTU\nptext\nnew item 9\n取書櫃1\nnew item 8\nnew item 17\n安南圖書區\nOUK\nSB3\nW-P\nEB-P\nz6bkf\nzd1e\n艾迪訊圖書館\n英才校區\n電子資源\n台中總館PBL專書區(限館內閱覽)\n台中總館績效暨獲獎區(限館內閱覽)\nnew item 14\nnew item 12\ncbook\nav\n北港分館視聽區\n北港分館流通櫃檯\n20230417\nnew item 4\nL\nz3llc\nH-EQ\nclp\n培德醫院\n北港附設醫院圖書室\n神資圖書館\nelect\nLB 圖書總館\nzdlf\nLIB\nzd1a2\n台中總館參考壁櫃\n台中總館B3裝訂期刊區\n台中總館視聽區(限館內閱覽)\n台中總館人文專書區-醫學法律(限館內閱覽)\n中醫醫史文獻室(限所內閱覽)\n台中總館技服組\n培德醫院圖書區\nnew item 11\n2\n北港附設醫院\n台北分院圖書區\n台北分院期刊區\n台北分院\nnew item 5\nH-MR\n五樓漫畫書專區\n實體館藏\n123\nBCSB4\nnew item 2\nCMUL - 中國醫藥大學圖書館')

WebUI.takeFullPageScreenshotAsCheckpoint('檢視新增證卡')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/新增一筆讀者證卡/a__1_2_3_4_5_6'))

WebUI.closeBrowser()

