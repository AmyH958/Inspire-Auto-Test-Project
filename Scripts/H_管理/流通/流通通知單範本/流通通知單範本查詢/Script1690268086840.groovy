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

WebUI.setText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_member_id (1)'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_member_pwd (1)'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input_(15trunk)_Submit (1)'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a_ (1)'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1 (1)'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2 (1)'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/div_ (1)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/div_ (1)'), '管理 > 流通 > 流通通知單範本')

WebUI.rightClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_1'))

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_1'), '1')

WebUI.verifyElementVisible(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a_(Due notice first)'))

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a_(Due notice first)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td_ (1)'), '欄位名稱:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1 (1)'), '通知單類型:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1_2 (1)'), '寄送方式:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/td__1_2_3 (1)'), '預設值:')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/input__TextField (1)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/div_ (1)'), '管理 > 流通 > 流通通知單範本')

WebUI.scrollToElement(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/div_ (1)'), 0)

WebUI.takeFullPageScreenshotAsCheckpoint('流通通知單範本查詢')

WebUI.scrollToElement(findTestObject('Page_(15trunk)/a__1_2_3'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3 (1)'), '取消')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3 (1)'))

WebUI.takeFullPageScreenshotAsCheckpoint('流通通知單範本查詢2')

WebUI.click(findTestObject('Object Repository/管理/流通/流通通知單範本/流通通知單範本查詢/a__1_2_3_4 (1)'))

WebUI.closeBrowser()

