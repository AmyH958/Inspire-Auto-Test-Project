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

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a_'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_(15trunk)/a__1'), 0)

WebUI.verifyElementText(findTestObject('Object Repository/Page_(15trunk)/a__1'), '書單維護')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1'))

WebUI.mouseOver(findTestObject('Object Repository/Page_(15trunk)/a__1_2'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_(15trunk)/a__1_2'), 0)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2'))

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/input__TextField_0'), '白雪壞公主(測試)')

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/textarea__TextArea'), '有夠壞的公主')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/input__elementName'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/img__Any_0_9'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/img__jwindow.dlocations1'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a_e-resources -'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.mouseOver(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'), '修改/存檔')

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'), 0)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('Page_(15trunk)/a__1_2_3_4'))

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3_4'))

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/textarea_Text In Opac_TextArea_0'), 'aaa')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('Page_(15trunk)/span_'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.scrollToElement(findTestObject('Page_(15trunk)/span_'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Page_(15trunk)/span_'), '確定')

WebUI.delay(1)

WebUI.mouseOver(findTestObject('Page_(15trunk)/span_'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/span_'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/input__dateBegin'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/img__datePickerImg'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/td_20'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/td_1'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/tr_'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/input__dateEnd'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/tr__1'))

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/input__dateEnd'), '2023/12/20')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_(15trunk)/td_()'), '白雪壞公主(測試)')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/div_Copyright 2016 iNspire 4.4.0-SNAPSHOT b_08b585'))

WebUI.takeFullPageScreenshotAsCheckpoint('Book Maintain List')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1_2_3_4_5'))

WebUI.closeBrowser()

