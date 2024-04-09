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

WebUI.setText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/input_(15trunk)_Submit'))

WebUI.enableSmartWait()

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a__1'), '一般設定')

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a__1_2'), '條碼號規範')

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/span_'), '條碼號類別篩選:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), '視聽資料\n圖書\n其他資料\n電子書\n期刊\nERM_網路資源\nERM_電子期刊\nERM_電子書\nERM_電子資料庫\n小冊子專用\n前置碼測試\n台北大電子資源\n123\n123\n123\n123')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), 
    '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), '視聽資料\n圖書\n其他資料\n電子書\n期刊\nERM_網路資源\nERM_電子期刊\nERM_電子書\nERM_電子資料庫\n小冊子專用\n前置碼測試\n台北大電子資源\n123\n123\n123\n123')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), 
    '3', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_ERM_ERM_ERM_ERM_123123123123'), 
    '0', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/span__1'), '館藏地篩選:')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '5', true)

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '北港分館視聽區(限館內閱覽)\n台中總館書庫\n台中總館閱覽組\n台中總館流通櫃檯\n台中總館博碩士論文區\n台中總館視聽區(獨立專區)\n台中總館研究計劃專書\n台中總館密閉書庫\n台中總館本校教職論著\n台中總館人文專書區-醫療品質(限館內閱覽)\nnew item 7\n台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)\n台中總館特藏室\n台中總館\nnew item 13\n安南醫院\n龍華科技大學圖書館\n豐原分院圖書區\n豐原分院\n語文教學中心圖書室\n語文教學中心\nT-P\nnew item 10\nL40\n20230418\nnew item 16\nnew item 3\nnew item 1\nLB-S\nOUK\nSB3\nW-P\nEB-P\nz6bkf\nzd1e\n艾迪訊圖書館\n英才校區\n電子資源\n台中總館探索史懷哲之路專書區\n北港分館書庫\n北港分館視聽區\n北港分館流通櫃檯\nnew item 17\n北港分館指參(限館內閱覽)\n台中總館新書展示區\n台中總館參考室\n台中總館本校教師升等資料區\n台中總館國考書區(限館內閱覽)\n台中總館線上資料\n台中總館教師指定參考書(限館內閱覽)\n台中總館期刊區\n台中總館視聽區\n台中總館人文專書區-醫療史(限館內閱覽)\n台中總館本校教職優良教材區\nnew item 14\nnew item 12\ncbook\nav\n台中總館績效暨獲獎區(限館內閱覽)\n台中總館期刊複本櫃\n台中總館PBL專書區(限館內閱覽)\n安南圖書區\n北港分館\n取書櫃2\nB007\ncircd\nNPTU\nptext\nnew item 9\n取書櫃1\nnew item 8\n20230417\nelect\nLB 圖書總館\nzdlf\nLIB\nzd1a2\nnew item 4\nL\nz3llc\nH-EQ\nclp\n培德醫院\n北港附設醫院圖書室\n神資圖書館\n台中總館參考壁櫃\n台中總館B3裝訂期刊區\n台中總館視聽區(限館內閱覽)\n台中總館人文專書區-醫學法律(限館內閱覽)\n中醫醫史文獻室(限所內閱覽)\n台中總館技服組\n培德醫院圖書區\nnew item 11\n2\n北港附設醫院\n台北分院圖書區\n台北分院期刊區\n台北分院\nnew item 5\nH-MR\n五樓漫畫書專區\n實體館藏\n123\nBCSB4\nnew item 2\nCMUL - 中國醫藥大學圖書館')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/select_()()-()new item 7-()-()new item 13T-_ececac'), 
    '8', true)

WebUI.takeFullPageScreenshotAsCheckpoint('條碼號規範查詢')

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/條碼號規範/條碼號規範查詢/a__1_2_3'))

WebUI.enableSmartWait()

WebUI.closeBrowser()

