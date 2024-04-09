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

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/input_(15trunk)_Submit'))

WebUI.maximizeWindow()

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/td_1234567890'), '1234567890')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a_NumberNational ID number'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/a__1_2_3'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3'), '刪除')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/div_'), '您確定要刪除?')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3_4_5'), '取消')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3'), '刪除')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/div_'), '您確定要刪除?')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/span__1'), '刪除')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/span__1'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/td__1'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/td__1'), '規則:')

WebUI.takeFullPageScreenshotAsCheckpoint('刪除讀者證卡成功')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/刪除一筆讀者證卡/a__1_2_3_4_5_6'))

WebUI.closeBrowser()

