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

WebUI.enableSmartWait()

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.setText(findTestObject('Object Repository/管理/編目/MARC規範查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/編目/MARC規範查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/編目/MARC規範查詢/input_(15trunk)_Submit'))

WebUI.scrollToElement(findTestObject('管理/編目/MARC規範查詢/a_'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/a_'), '管理')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/a_'))

WebUI.scrollToElement(findTestObject('管理/編目/MARC規範查詢/a__1'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/a__1'), '編目')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/a__1'))

WebUI.scrollToElement(findTestObject('管理/編目/MARC規範查詢/a_MARC'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/a_MARC'), 'MARC規範')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/a_MARC'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/div_MARC'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/div_MARC'), '管理 > 編目 > MARC規範')

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/span_MARC'), '選擇 MARC 群組')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/td_CHMARCCNMARCFRBRMARC21SYSTEMUNIMARC'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/編目/MARC規範查詢/span_MARC_1'), '選擇 MARC 格式')

WebUI.takeFullPageScreenshotAsCheckpoint('MARC規範查詢')

WebUI.enhancedClick(findTestObject('Object Repository/管理/編目/MARC規範查詢/a_ (1)'))

WebUI.closeBrowser()

