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

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a__1_2'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/div_1(Due notice first)2(Due notice repeat)_1310ca'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/td__EditArea'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a_(Due notice first)'), '圖書即將到期提醒通知(Due notice first)')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a__20230725'), '測試流通範本_20230725')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/td__20230725'))

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/img__20230725_Any_17'), 0)

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/img__20230725_Any_17'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/div_'), '您確定要刪除這筆紀錄?')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/span_'), '否')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a__20230725'), '測試流通範本_20230725')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/img__20230725_Any_17'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/div_'), '您確定要刪除這筆紀錄?')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/span__1'), '是')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/span__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/div_1(Due notice first)2(Due notice repeat)_23324f'))

WebUI.takeFullPageScreenshotAsCheckpoint('刪除一筆流通通知單範本')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/刪除流通通知單範本/a__1_2_3'))

WebUI.closeBrowser()

